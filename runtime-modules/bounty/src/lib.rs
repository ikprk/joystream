//! This pallet works with crowd funded bounties that allows a member, or the council, to crowd
//! fund work on projects with a public benefit.
//!
//! ### Bounty stages
//! - Funding - a bounty is being funded.
//! - WorkSubmission - interested participants can submit their work.
//! - Judgment - working periods ended and the oracle should provide their judgment.
//! - Withdrawal - all funds can be withdrawn.
//!
//! A detailed description could be found [here](https://github.com/Joystream/joystream/issues/1998).
//!
//! ### Supported extrinsics:
//! - [create_bounty](./struct.Module.html#method.create_bounty) - creates a bounty
//! - [cancel_bounty](./struct.Module.html#method.cancel_bounty) - cancels a bounty
//! - [veto_bounty](./struct.Module.html#method.veto_bounty) - vetoes a bounty
//! - [fund_bounty](./struct.Module.html#method.fund_bounty) - provide funding for a bounty
//! - [withdraw_funding](./struct.Module.html#method.withdraw_funding) - withdraw
//! funding for a failed bounty.
//! - [withdraw_creator_cherry](./struct.Module.html#method.withdraw_creator_cherry) - withdraw
//! a cherry for a failed or canceled bounty.
//! - [announce_work_entry](./struct.Module.html#method.announce_work_entry) - announce
//! work entry for a successful bounty.
//! - [withdraw_work_entry](./struct.Module.html#method.withdraw_work_entry) - withdraw
//! work entry for a bounty.
//! - [submit_work](./struct.Module.html#method.submit_work) - submit work for a bounty.
//! - [submit_oracle_judgment](./struct.Module.html#method.submit_oracle_judgment) - submits an
//! oracle judgment for a bounty.
//! - [withdraw_work_entrant_funds](./struct.Module.html#method.withdraw_work_entrant_funds) -
//! withdraw work entrant funds.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub(crate) mod tests;

mod actors;
mod stages;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// TODO: ensure document compatibility
// TODO: full successful test with all withdrawals
// TODO: make sure cherry goes back to creator on successful bounty.
// TODO: add working stake unstaking period.
// TODO: test all stages
// TODO: withdraw member funding if there is no judgment.
// TODO: withdraw funds for winners.
// TODO - open question:
/* if you just store winners and to_be_slashed, and you do no iteration even on
these sets, but instead require active action from everyone, then you may perhaps not even need to
limit the set size? You could store thousands of ids in either of those values without impacting
read io speed, as we have seen, so could be fine. */
// TODO - open question: should we have exactly 100% reward distribution for the winners?
// we could allow less than 100% but what to do with the rest of the bounty pool?
// It could be a problem because the remaining sum could be very low to be an incentive to withdraw
// for funders. Should we introduce a remaining sum threshold that affects the "burning decision"?

/// pallet_bounty WeightInfo.
/// Note: This was auto generated through the benchmark CLI using the `--weight-trait` flag
pub trait WeightInfo {
    fn create_bounty_by_council() -> Weight;
    fn create_bounty_by_member() -> Weight;
    fn cancel_bounty_by_member() -> Weight;
    fn cancel_bounty_by_council() -> Weight;
    fn veto_bounty() -> Weight;
    fn fund_bounty_by_member() -> Weight;
    fn fund_bounty_by_council() -> Weight;
    fn withdraw_funding_by_member() -> Weight;
    fn withdraw_funding_by_council() -> Weight;
    fn withdraw_creator_cherry_by_council() -> Weight;
    fn withdraw_creator_cherry_by_member() -> Weight;
    fn announce_work_entry() -> Weight;
    fn withdraw_work_entry() -> Weight;
    fn submit_work(i: u32) -> Weight;
    fn submit_oracle_judgment_by_council_all_winners(i: u32) -> Weight;
    fn submit_oracle_judgment_by_council_all_rejected(i: u32) -> Weight;
    fn submit_oracle_judgment_by_member_all_winners(i: u32) -> Weight;
    fn submit_oracle_judgment_by_member_all_rejected(i: u32) -> Weight;
}

type WeightInfoBounty<T> = <T as Trait>::WeightInfo;

pub(crate) use actors::BountyActorManager;
pub(crate) use stages::BountyStageCalculator;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::{Currency, ExistenceRequirement, Get};
use frame_support::weights::Weight;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter};
use frame_system::ensure_root;
use sp_arithmetic::traits::{Saturating, Zero};
use sp_runtime::{
    traits::{AccountIdConversion, Hash},
    ModuleId,
};
use sp_runtime::{Perbill, SaturatedConversion};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::vec::Vec;

use common::council::CouncilBudgetManager;
use common::origin::MemberOriginValidator;
use common::{MemberId, StakingAccountValidator};
use staking_handler::StakingHandler;

/// Main pallet-bounty trait.
pub trait Trait: frame_system::Trait + balances::Trait + common::Trait {
    /// Events
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The bounty's module id, used for deriving its sovereign account ID.
    type ModuleId: Get<ModuleId>;

    /// Bounty Id type
    type BountyId: From<u32> + Parameter + Default + Copy;

    /// Validates staking account ownership for a member.
    type StakingAccountValidator: common::StakingAccountValidator<Self>;

    /// Validates member ID and origin combination.
    type MemberOriginValidator: MemberOriginValidator<Self::Origin, MemberId<Self>, Self::AccountId>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;

    /// Provides an access for the council budget.
    type CouncilBudgetManager: CouncilBudgetManager<BalanceOf<Self>>;

    /// Provides stake logic implementation.
    type StakingHandler: StakingHandler<Self::AccountId, BalanceOf<Self>, MemberId<Self>>;

    /// Work entry Id type
    type WorkEntryId: From<u32> + Parameter + Default + Copy + Ord;

    /// Defines max work entry number for a bounty.
    /// It limits further work entries iteration after the judge decision about winners, non-winners
    /// and "byzantine" (malicious) users.
    type MaxWorkEntryLimit: Get<u32>;

    /// Defines min cherry for a bounty.
    type MinCherryLimit: Get<BalanceOf<Self>>;

    /// Defines min funding amount for a bounty.
    type MinFundingLimit: Get<BalanceOf<Self>>;
}

/// Alias type for the BountyParameters.
pub type BountyCreationParameters<T> = BountyParameters<
    BalanceOf<T>,
    <T as frame_system::Trait>::BlockNumber,
    <T as common::Trait>::MemberId,
>;

/// Defines who can submit the work.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum AssuranceContractType<MemberId: Ord> {
    /// Anyone can submit the work.
    Open,

    /// Only specific members can submit the work.
    Closed(BTreeSet<MemberId>),
}

impl<MemberId: Ord> Default for AssuranceContractType<MemberId> {
    fn default() -> Self {
        AssuranceContractType::Open
    }
}

/// Defines parameters for the bounty creation.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct BountyParameters<Balance, BlockNumber, MemberId: Ord> {
    /// Origin that will select winner(s), is either a given member or a council.
    pub oracle: BountyActor<MemberId>,

    /// Contract type defines who can submit the work.
    pub contract_type: AssuranceContractType<MemberId>,

    /// Bounty creator: could be a member or a council.
    pub creator: BountyActor<MemberId>,

    /// An mount of funding, possibly 0, provided by the creator which will be split among all other
    /// contributors should the min funding bound not be reached. If reached, cherry is returned to
    /// the creator. When council is creating bounty, this comes out of their budget, when a member
    /// does it, it comes from an account.
    pub cherry: Balance,

    /// The minimum total quantity of funds, possibly 0, required for the bounty to become
    /// available for people to work on.
    pub min_amount: Balance,

    /// Maximum funding accepted, if this limit is reached, funding automatically is over.
    pub max_amount: Balance,

    /// Amount of stake required, possibly 0, to enter bounty as entrant.
    pub entrant_stake: Balance,

    /// Number of blocks from creation until funding is no longer possible. If not provided, then
    /// funding is called perpetual, and it only ends when minimum amount is reached.
    pub funding_period: Option<BlockNumber>,

    /// Number of blocks from end of funding period until people can no longer submit
    /// bounty submissions.
    pub work_period: BlockNumber,

    /// Number of block from end of work period until oracle can no longer decide winners.
    pub judging_period: BlockNumber,
}

/// Bounty actor to perform operations for a bounty.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum BountyActor<MemberId> {
    /// Council performs operations for a bounty.
    Council,

    /// Member performs operations for a bounty.
    Member(MemberId),
}

impl<MemberId> Default for BountyActor<MemberId> {
    fn default() -> Self {
        BountyActor::Council
    }
}

/// Defines current bounty stage.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum BountyStage {
    /// Bounty founding stage.
    Funding {
        /// Bounty has already some contributions.
        has_contributions: bool,
    },

    /// A bounty has gathered necessary funds and ready to accept work submissions.
    WorkSubmission,

    /// Working periods ended and the oracle should provide their judgment.
    Judgment,

    /// Funding and cherry can be withdrawn.
    Withdrawal {
        /// Creator cherry is not withdrawn and greater than zero.
        cherry_needs_withdrawal: bool,
    },
}

/// Defines current bounty state.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum BountyMilestone<BlockNumber> {
    /// Bounty was created at given block number.
    /// Boolean value defines whether the bounty has some funding contributions.
    Created {
        /// Bounty creation block.
        created_at: BlockNumber,
        /// Bounty has already some contributions.
        has_contributions: bool,
    },

    /// A bounty was canceled.
    Canceled,

    /// A bounty funding was successful and it exceeded max funding amount.
    BountyMaxFundingReached {
        ///  A bounty funding was successful on the provided block.
        max_funding_reached_at: BlockNumber,
    },

    /// Some work was submitted for a bounty.
    WorkSubmitted {
        ///  Starting block for the work period.
        work_period_started_at: BlockNumber,
    },

    /// A judgment was submitted for a bounty.
    JudgmentSubmitted {
        /// The bounty judgment contains at least a single winner.
        successful_bounty: bool,
    },

    /// Creator cherry was withdrawn.
    CreatorCherryWithdrawn,
}

impl<BlockNumber: Default> Default for BountyMilestone<BlockNumber> {
    fn default() -> Self {
        BountyMilestone::Created {
            created_at: Default::default(),
            has_contributions: false,
        }
    }
}

/// Alias type for the Bounty.
pub type Bounty<T> = BountyRecord<
    BalanceOf<T>,
    <T as frame_system::Trait>::BlockNumber,
    <T as common::Trait>::MemberId,
>;

/// Crowdfunded bounty record.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct BountyRecord<Balance, BlockNumber, MemberId: Ord> {
    /// Bounty creation parameters.
    pub creation_params: BountyParameters<Balance, BlockNumber, MemberId>,

    /// Total funding balance reached so far.
    /// Includes initial funding by a creator and other members funding.
    pub total_funding: Balance,

    /// Bounty current milestone(state). It represents fact known about the bounty, eg.:
    /// it was canceled or max funding amount was reached.
    pub milestone: BountyMilestone<BlockNumber>,

    /// Current active work entry counter.
    pub active_work_entry_count: u32,
}

impl<Balance, BlockNumber, MemberId: Ord> BountyRecord<Balance, BlockNumber, MemberId> {
    // Increments bounty active work entry counter.
    fn increment_active_work_entry_counter(&mut self) {
        self.active_work_entry_count += 1;
    }

    // Decrements bounty active work entry counter. Nothing happens on zero counter.
    fn decrement_active_work_entry_counter(&mut self) {
        if self.active_work_entry_count > 0 {
            self.active_work_entry_count -= 1;
        }
    }
}

/// Alias type for the WorkEntry.
pub type WorkEntry<T> = WorkEntryRecord<
    <T as frame_system::Trait>::AccountId,
    <T as common::Trait>::MemberId,
    <T as frame_system::Trait>::BlockNumber,
    BalanceOf<T>,
>;

/// Work entry.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct WorkEntryRecord<AccountId, MemberId, BlockNumber, Balance> {
    /// Work entrant member ID.
    pub member_id: MemberId,

    /// Optional account ID for staking lock.
    pub staking_account_id: Option<AccountId>,

    /// Account ID for reward.
    pub reward_account_id: AccountId,

    /// Work entry submission block.
    pub submitted_at: BlockNumber,

    /// Last submitted work data hash.
    pub last_submitted_work: Option<Vec<u8>>,

    /// Oracle judgment for the work entry.
    pub oracle_judgment_result: OracleWorkEntryJudgment<Balance>,
}

/// Defines the oracle judgment for the work entry.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum OracleWorkEntryJudgment<Balance> {
    /// The work entry is selected as a winner.
    Winner { reward: Balance },

    /// The work entry is not selected as a winner but no penalties applies.
    Legit,

    /// The work entry is considered harmful. The stake will be slashed.
    Rejected,
}

impl<Balance> Default for OracleWorkEntryJudgment<Balance> {
    fn default() -> Self {
        Self::Legit
    }
}

impl<Balance> OracleWorkEntryJudgment<Balance> {
    // Work entry judgment helper. Returns true for winners.
    pub(crate) fn is_winner(&self) -> bool {
        matches!(*self, Self::Winner { .. })
    }
}

/// Balance alias for `balances` module.
pub type BalanceOf<T> = <T as balances::Trait>::Balance;

// Entrant stake helper struct.
struct RequiredStakeInfo<T: Trait> {
    // stake amount
    amount: BalanceOf<T>,
    // staking_account_id
    account_id: T::AccountId,
}

/// An alias for the OracleJudgment.
pub type OracleJudgmentOf<T> = OracleJudgment<<T as Trait>::WorkEntryId, BalanceOf<T>>;

/// The collection of the oracle judgments for the work entries.
pub type OracleJudgment<WorkEntryId, Balance> =
    BTreeMap<WorkEntryId, OracleWorkEntryJudgment<Balance>>;

decl_storage! {
    trait Store for Module<T: Trait> as Bounty {
        /// Bounty storage.
        pub Bounties get(fn bounties) : map hasher(blake2_128_concat) T::BountyId => Bounty<T>;

        /// Double map for bounty funding. It stores a member or council funding for bounties.
        pub BountyContributions get(fn contribution_by_bounty_by_actor): double_map
            hasher(blake2_128_concat) T::BountyId,
            hasher(blake2_128_concat) BountyActor<MemberId<T>> => BalanceOf<T>;

        /// Count of all bounties that have been created.
        pub BountyCount get(fn bounty_count): u32;

        /// Work entry storage double map.
        pub WorkEntries get(fn work_entries): double_map
            hasher(blake2_128_concat) T::BountyId,
            hasher(blake2_128_concat) T::WorkEntryId => WorkEntry<T>;

        /// Count of all work entries that have been created.
        pub WorkEntryCount get(fn work_entry_count): u32;
    }
}

decl_event! {
    pub enum Event<T>
    where
        <T as Trait>::BountyId,
        <T as Trait>::WorkEntryId,
        Balance = BalanceOf<T>,
        MemberId = MemberId<T>,
        <T as frame_system::Trait>::AccountId,
        BountyCreationParameters = BountyCreationParameters<T>,
        OracleJudgment = OracleJudgmentOf<T>,
    {
        /// A bounty was created.
        BountyCreated(BountyId, BountyCreationParameters),

        /// A bounty was canceled.
        BountyCanceled(BountyId, BountyActor<MemberId>),

        /// A bounty was vetoed.
        BountyVetoed(BountyId),

        /// A bounty was funded by a member or a council.
        BountyFunded(BountyId, BountyActor<MemberId>, Balance),

        /// A bounty has reached its maximum funding amount.
        BountyMaxFundingReached(BountyId),

        /// A member or a council has withdrawn the funding.
        BountyFundingWithdrawal(BountyId, BountyActor<MemberId>),

        /// A bounty creator has withdrew the funding (member or council).
        BountyCreatorFundingWithdrawal(BountyId, BountyActor<MemberId>),

        /// A bounty was removed.
        BountyRemoved(BountyId),

        /// Work entry was announced.
        /// Params:
        /// - bounty ID
        /// - created entry ID
        /// - entrant member ID
        /// - reward account ID
        /// - optional staking account ID
        WorkEntryAnnounced(BountyId, WorkEntryId, MemberId, AccountId, Option<AccountId>),

        /// Work entry was withdrawn.
        /// Params:
        /// - bounty ID
        /// - entry ID
        /// - entrant member ID
        WorkEntryWithdrawn(BountyId, WorkEntryId, MemberId),

        /// Work entry was slashed.
        /// Params:
        /// - bounty ID
        /// - entry ID
        WorkEntrySlashed(BountyId, WorkEntryId),

        /// Submit work.
        /// Params:
        /// - bounty ID
        /// - created entry ID
        /// - entrant member ID
        /// - work data (description, URL, BLOB, etc.)
        WorkSubmitted(BountyId, WorkEntryId, MemberId, Vec<u8>),

        /// Submit oracle judgment.
        /// Params:
        /// - bounty ID
        /// - oracle
        /// - judgment data
        OracleJudgmentSubmitted(BountyId, BountyActor<MemberId>, OracleJudgment),

        /// Work entry was slashed.
        /// Params:
        /// - bounty ID
        /// - entry ID
        /// - entrant member ID
        WorkEntrantFundsWithdrawn(BountyId, WorkEntryId, MemberId),
    }
}

decl_error! {
    /// Bounty pallet predefined errors
    pub enum Error for Module<T: Trait> {
        /// Min funding amount cannot be greater than max amount.
        MinFundingAmountCannotBeGreaterThanMaxAmount,

        /// Bounty doesnt exist.
        BountyDoesntExist,

        /// Operation can be performed only by a bounty creator.
        NotBountyActor,

        /// Work period cannot be zero.
        WorkPeriodCannotBeZero,

        /// Judging period cannot be zero.
        JudgingPeriodCannotBeZero,

        /// Unexpected bounty stage for an operation: Funding.
        InvalidStageUnexpectedFunding,

        /// Unexpected bounty stage for an operation: WorkSubmission.
        InvalidStageUnexpectedWorkSubmission,

        /// Unexpected bounty stage for an operation: Judgment.
        InvalidStageUnexpectedJudgment,

        /// Unexpected bounty stage for an operation: Withdrawal.
        InvalidStageUnexpectedWithdrawal,

        /// Insufficient balance for a bounty cherry.
        InsufficientBalanceForBounty,

        /// Funding period is not expired for the bounty.
        FundingPeriodNotExpired,

        /// Cannot found bounty contribution.
        NoBountyContributionFound,

        /// There is nothing to withdraw.
        NothingToWithdraw,

        /// Incorrect funding amount.
        ZeroFundingAmount,

        /// There is not enough balance for a stake.
        InsufficientBalanceForStake,

        /// The conflicting stake discovered. Cannot stake.
        ConflictingStakes,

        /// No staking account was provided.
        NoStakingAccountProvided,

        /// Work entry doesnt exist.
        WorkEntryDoesntExist,

        /// Cannot add work entry because of the limit.
        MaxWorkEntryLimitReached,

        /// Cherry less then minimum allowed.
        CherryLessThenMinimumAllowed,

        /// Funding amount less then minimum allowed.
        FundingLessThenMinimumAllowed,

        /// Incompatible assurance contract type for a member: cannot submit work to the 'closed
        /// assurance' bounty contract.
        CannotSubmitWorkToClosedContractBounty,

        /// Cannot create a 'closed assurance contract' bounty with empty member list.
        ClosedContractMemberListIsEmpty,

        /// Cannot create a 'closed assurance contract' bounty with member list larger
        /// than allowed max work entry limit.
        ClosedContractMemberListIsTooLarge,

        /// Staking account doesn't belong to a member.
        InvalidStakingAccountForMember,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Emits an event. Default substrate implementation.
        fn deposit_event() = default;

        /// Exports const - max work entry number for a bounty.
        const MaxWorkEntryLimit: u32 = T::MaxWorkEntryLimit::get();

        /// Exports const - min cherry value limit for a bounty.
        const MinCherryLimit: BalanceOf<T> = T::MinCherryLimit::get();

        /// Exports const - min funding amount limit for a bounty.
        const MinFundingLimit: BalanceOf<T> = T::MinFundingLimit::get();

        /// Creates a bounty. Metadata stored in the transaction log but discarded after that.
        /// <weight>
        ///
        /// ## Weight
        /// `O (W)` where:
        /// - `W` is the _metadata length.
        /// - DB:
        ///    - O(1)
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::create_bounty_by_member()
              .max(WeightInfoBounty::<T>::create_bounty_by_council())]
        pub fn create_bounty(origin, params: BountyCreationParameters<T>, _metadata: Vec<u8>) {
            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor(
                origin,
                params.creator.clone()
            )?;

            Self::ensure_create_bounty_parameters_valid(&params)?;

            bounty_creator_manager.validate_balance_sufficiency(params.cherry)?;

            //
            // == MUTATION SAFE ==
            //

            let next_bounty_count_value = Self::bounty_count() + 1;
            let bounty_id = T::BountyId::from(next_bounty_count_value);

            bounty_creator_manager.transfer_funds_to_bounty_account(bounty_id, params.cherry)?;

            let created_bounty_milestone = BountyMilestone::Created {
                created_at: Self::current_block(),
                has_contributions: false, // just created - no contributions
            };

            let bounty = Bounty::<T> {
                total_funding: Zero::zero(),
                creation_params: params.clone(),
                milestone: created_bounty_milestone,
                active_work_entry_count: 0,
            };

            <Bounties<T>>::insert(bounty_id, bounty);
            BountyCount::mutate(|count| {
                *count = next_bounty_count_value
            });
            Self::deposit_event(RawEvent::BountyCreated(bounty_id, params));
        }

        /// Cancels a bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::cancel_bounty_by_member()
              .max(WeightInfoBounty::<T>::cancel_bounty_by_council())]
        pub fn cancel_bounty(origin, creator: BountyActor<MemberId<T>>, bounty_id: T::BountyId) {
            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor(
                origin,
                creator.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            bounty_creator_manager.validate_actor(&bounty.creation_params.creator)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(
                current_bounty_stage,
                BountyStage::Funding { has_contributions: false }
            )?;

            //
            // == MUTATION SAFE ==
            //

            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::Canceled;
            });

            Self::deposit_event(RawEvent::BountyCanceled(bounty_id, creator));
        }

        /// Vetoes a bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::veto_bounty()]
        pub fn veto_bounty(origin, bounty_id: T::BountyId) {
            ensure_root(origin)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(
                current_bounty_stage,
                BountyStage::Funding { has_contributions: false }
            )?;

            //
            // == MUTATION SAFE ==
            //

            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::Canceled;
            });

            Self::deposit_event(RawEvent::BountyVetoed(bounty_id));
        }

        /// Provides bounty funding.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::fund_bounty_by_member()
              .max(WeightInfoBounty::<T>::fund_bounty_by_council())]
        pub fn fund_bounty(
            origin,
            funder: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
            amount: BalanceOf<T>
        ) {
            let bounty_funder_manager = BountyActorManager::<T>::get_bounty_actor(
                origin,
                funder.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            ensure!(amount > Zero::zero(), Error::<T>::ZeroFundingAmount);

            ensure!(amount >= T::MinFundingLimit::get(), Error::<T>::FundingLessThenMinimumAllowed);

            bounty_funder_manager.validate_balance_sufficiency(amount)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);
            ensure!(
                matches!(current_bounty_stage, BountyStage::Funding{..}),
                Self::unexpected_bounty_stage_error(current_bounty_stage),
            );

            //
            // == MUTATION SAFE ==
            //

            bounty_funder_manager.transfer_funds_to_bounty_account(bounty_id, amount)?;

            let total_funding = bounty.total_funding.saturating_add(amount);
            let maximum_funding_reached = total_funding >= bounty.creation_params.max_amount;
            let new_milestone = Self::get_bounty_milestone_on_funding(
                    maximum_funding_reached,
                    bounty.milestone
            );

            // Update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.total_funding = total_funding;
                bounty.milestone = new_milestone;
            });

            // Update member funding record checking previous funding.
            let funds_so_far = Self::contribution_by_bounty_by_actor(bounty_id, &funder);
            let total_funding = funds_so_far.saturating_add(amount);
            <BountyContributions<T>>::insert(bounty_id, funder.clone(), total_funding);

            // Fire events.
            Self::deposit_event(RawEvent::BountyFunded(bounty_id, funder, amount));
            if  maximum_funding_reached{
                Self::deposit_event(RawEvent::BountyMaxFundingReached(bounty_id));
            }
        }

        /// Withdraw bounty funding by a member or a council.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::withdraw_funding_by_member()
              .max(WeightInfoBounty::<T>::withdraw_funding_by_council())]
        pub fn withdraw_funding(
            origin,
            funder: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
        ) {
            let bounty_funder_manager = BountyActorManager::<T>::get_bounty_actor(
                origin,
                funder.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);
            Self::ensure_bounty_withdrawal_stage(current_bounty_stage)?;

            ensure!(
                <BountyContributions<T>>::contains_key(&bounty_id, &funder),
                Error::<T>::NoBountyContributionFound,
            );

            let funding_amount = <BountyContributions<T>>::get(&bounty_id, &funder);
            let cherry_fraction = Self::get_cherry_fraction_for_member(&bounty, funding_amount);
            let withdrawal_amount = funding_amount + cherry_fraction;

            //
            // == MUTATION SAFE ==
            //

            bounty_funder_manager.transfer_funds_from_bounty_account(bounty_id, withdrawal_amount)?;

            <BountyContributions<T>>::remove(&bounty_id, &funder);

            Self::deposit_event(RawEvent::BountyFundingWithdrawal(bounty_id, funder));

            if Self::withdrawal_completed(&current_bounty_stage, &bounty_id) {
                Self::remove_bounty(&bounty_id);
            }
        }

        /// Withdraw creator funding.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::withdraw_creator_cherry_by_member()
              .max(WeightInfoBounty::<T>::withdraw_creator_cherry_by_council())]
        pub fn withdraw_creator_cherry(
            origin,
            creator: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
        ) {
            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor(
                origin,
                creator.clone(),
            )?;

            let mut bounty = Self::ensure_bounty_exists(&bounty_id)?;

            bounty_creator_manager.validate_actor(&bounty.creation_params.creator)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            let cherry_needs_withdrawal =
                Self::ensure_bounty_withdrawal_stage(current_bounty_stage)?;

            ensure!(cherry_needs_withdrawal, Error::<T>::NothingToWithdraw);

            //
            // == MUTATION SAFE ==
            //

            let cherry = Self::get_cherry_for_creator_withdrawal(&bounty, current_bounty_stage);

            bounty_creator_manager.transfer_funds_from_bounty_account(bounty_id, cherry)?;

            bounty.milestone = BountyMilestone::CreatorCherryWithdrawn;
            <Bounties<T>>::insert(bounty_id, bounty.clone());

            Self::deposit_event(RawEvent::BountyCreatorFundingWithdrawal(bounty_id, creator));

            let new_bounty_stage = Self::get_bounty_stage(&bounty);

            if Self::withdrawal_completed(&new_bounty_stage, &bounty_id) {
                Self::remove_bounty(&bounty_id);
            }
        }

        /// Announce work entry for a successful bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::announce_work_entry()]
        pub fn announce_work_entry(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            reward_account_id: T::AccountId,
            staking_account_id: Option<T::AccountId>,
        ) {
            T::MemberOriginValidator::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::WorkSubmission)?;

            let stake = Self::validate_entrant_stake(
                member_id,
                &bounty,
                staking_account_id.clone()
            )?;

            ensure!(
                bounty.active_work_entry_count < T::MaxWorkEntryLimit::get(),
                Error::<T>::MaxWorkEntryLimitReached,
            );

            Self::ensure_valid_contract_type(&bounty, &member_id)?;

            //
            // == MUTATION SAFE ==
            //

            let next_entry_count_value = Self::work_entry_count() + 1;
            let entry_id = T::WorkEntryId::from(next_entry_count_value);

            // Lock stake balance for bounty if the stake is required.
            if let Some(stake) = stake {
                T::StakingHandler::lock(&stake.account_id, stake.amount);
            }

            let entry = WorkEntry::<T> {
                member_id,
                reward_account_id: reward_account_id.clone(),
                staking_account_id: staking_account_id.clone(),
                submitted_at: Self::current_block(),
                last_submitted_work: None,
                // The default initial value.
                oracle_judgment_result: OracleWorkEntryJudgment::Legit,
            };

            <WorkEntries<T>>::insert(bounty_id, entry_id, entry);
            WorkEntryCount::mutate(|count| {
                *count = next_entry_count_value
            });

            // Increment work entry counter and update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.increment_active_work_entry_counter();
            });

            Self::deposit_event(RawEvent::WorkEntryAnnounced(
                bounty_id,
                entry_id,
                member_id,
                reward_account_id,
                staking_account_id,
            ));
        }

        /// Withdraw work entry for a bounty. Existing stake could be partially slashed.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::withdraw_work_entry()]
        pub fn withdraw_work_entry(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            entry_id: T::WorkEntryId,
        ) {
            T::MemberOriginValidator::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::WorkSubmission)?;

            let entry = Self::ensure_work_entry_exists(&bounty_id, &entry_id)?;

            //
            // == MUTATION SAFE ==
            //

            Self::unlock_work_entry_stake(&bounty, &entry);

            Self::remove_work_entry(&bounty_id, &entry_id);

            Self::deposit_event(RawEvent::WorkEntryWithdrawn(bounty_id, entry_id, member_id));
        }

        /// Submit work for a bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (N)`
        /// - `N` is the work_data length,
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight =  WeightInfoBounty::<T>::submit_work(work_data.len().saturated_into())]
        pub fn submit_work(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            entry_id: T::WorkEntryId,
            work_data: Vec<u8>
        ) {
            T::MemberOriginValidator::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::WorkSubmission)?;

            Self::ensure_work_entry_exists(&bounty_id, &entry_id)?;

            //
            // == MUTATION SAFE ==
            //

            let hashed = T::Hashing::hash(&work_data);
            let work_data_hash = hashed.as_ref().to_vec();

            // Update entry
            <WorkEntries<T>>::mutate(bounty_id, entry_id, |entry| {
                entry.last_submitted_work = Some(work_data_hash);
            });

            let new_milestone = Self::get_bounty_milestone_on_work_submitting(&bounty);

            // Update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = new_milestone;
            });

            Self::deposit_event(RawEvent::WorkSubmitted(bounty_id, entry_id, member_id, work_data));
        }

        /// Submits an oracle judgment for a bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (N)`
        /// - `N` is the work_data length,
        /// - db:
        ///    - `O(N)`
        /// # </weight>
        #[weight = Module::<T>::submit_oracle_judgement_weight(&judgment)]
        pub fn submit_oracle_judgment(
            origin,
            oracle: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
            judgment: OracleJudgment<T::WorkEntryId, BalanceOf<T>>
        ) {
            let bounty_oracle_manager = BountyActorManager::<T>::get_bounty_actor(
                origin,
                oracle.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            bounty_oracle_manager.validate_actor(&bounty.creation_params.oracle)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::Judgment)?;

            Self::validate_judgment(&bounty_id, &judgment)?;

            // Lookup for any winners in the judgment.
            let successful_bounty = Self::judgment_has_winners(&judgment);

            //
            // == MUTATION SAFE ==
            //

            // Update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::JudgmentSubmitted {
                    successful_bounty
                };
            });

            // Judgments triage.
            for (entry_id, work_entry_judgment) in judgment.iter() {
                // Update work entries for winners and legitimate participants.
                if *work_entry_judgment != OracleWorkEntryJudgment::Rejected{
                    <WorkEntries<T>>::mutate(bounty_id, entry_id, |entry| {
                        entry.oracle_judgment_result = *work_entry_judgment;
                    });
                } else {
                    let entry = Self::work_entries(bounty_id, entry_id);

                    Self::slash_work_entry_stake(&entry);

                    Self::remove_work_entry(&bounty_id, &entry_id);

                    Self::deposit_event(RawEvent::WorkEntrySlashed(bounty_id, *entry_id));
                }
            }

            Self::deposit_event(RawEvent::OracleJudgmentSubmitted(bounty_id, oracle, judgment));
        }

        /// Withdraw work entrant funds.
        /// Both legitimate participants and winners get their stake unlocked. Winners also get a
        /// bounty reward.
        #[weight = 1000000] // TODO: adjust weight and its comment
        pub fn withdraw_work_entrant_funds(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            entry_id: T::WorkEntryId,
        ) {
            T::MemberOriginValidator::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_withdrawal_stage(current_bounty_stage)?;

            let entry = Self::ensure_work_entry_exists(&bounty_id, &entry_id)?;


            //
            // == MUTATION SAFE ==
            //

            // Claim the winner reward.
            if let OracleWorkEntryJudgment::Winner { reward } = entry.oracle_judgment_result {
                Self::transfer_funds_from_bounty_account(
                    &entry.reward_account_id,
                    bounty_id,
                    reward
                )?;
            }

            // Unstake the full work entry state.
            Self::unlock_work_entry_stake(&bounty, &entry);

            // Delete the work entry record from the storage.
            Self::remove_work_entry(&bounty_id, &entry_id);

            // Fire an event.
            Self::deposit_event(RawEvent::WorkEntrantFundsWithdrawn(bounty_id, entry_id, member_id));

            // Remove the bounty in case of the last withdrawal operation.
            if Self::withdrawal_completed(&current_bounty_stage, &bounty_id) {
                Self::remove_bounty(&bounty_id);
            }
        }
    }
}

impl<T: Trait> Module<T> {
    // Wrapper-function over System::block_number()
    pub(crate) fn current_block() -> T::BlockNumber {
        <frame_system::Module<T>>::block_number()
    }

    // Validates parameters for a bounty creation.
    fn ensure_create_bounty_parameters_valid(
        params: &BountyCreationParameters<T>,
    ) -> DispatchResult {
        ensure!(
            params.work_period != Zero::zero(),
            Error::<T>::WorkPeriodCannotBeZero
        );

        ensure!(
            params.judging_period != Zero::zero(),
            Error::<T>::JudgingPeriodCannotBeZero
        );

        ensure!(
            params.min_amount <= params.max_amount,
            Error::<T>::MinFundingAmountCannotBeGreaterThanMaxAmount
        );

        ensure!(
            params.cherry >= T::MinCherryLimit::get(),
            Error::<T>::CherryLessThenMinimumAllowed
        );

        if let AssuranceContractType::Closed(ref member_ids) = params.contract_type {
            ensure!(
                !member_ids.is_empty(),
                Error::<T>::ClosedContractMemberListIsEmpty
            );

            ensure!(
                member_ids.len() <= T::MaxWorkEntryLimit::get().saturated_into(),
                Error::<T>::ClosedContractMemberListIsTooLarge
            );
        }

        Ok(())
    }

    // Verifies that member balance is sufficient for a bounty.
    fn check_balance_for_account(amount: BalanceOf<T>, account_id: &T::AccountId) -> bool {
        balances::Module::<T>::usable_balance(account_id) >= amount
    }

    // Transfer funds from the member account to the bounty account.
    fn transfer_funds_to_bounty_account(
        account_id: &T::AccountId,
        bounty_id: T::BountyId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let bounty_account_id = Self::bounty_account_id(bounty_id);

        <balances::Module<T> as Currency<T::AccountId>>::transfer(
            account_id,
            &bounty_account_id,
            amount,
            ExistenceRequirement::KeepAlive,
        )
    }

    // Transfer funds from the bounty account to the member account.
    fn transfer_funds_from_bounty_account(
        account_id: &T::AccountId,
        bounty_id: T::BountyId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let bounty_account_id = Self::bounty_account_id(bounty_id);

        <balances::Module<T> as Currency<T::AccountId>>::transfer(
            &bounty_account_id,
            account_id,
            amount,
            ExistenceRequirement::KeepAlive,
        )
    }

    // Verifies bounty existence and retrieves a bounty from the storage.
    fn ensure_bounty_exists(bounty_id: &T::BountyId) -> Result<Bounty<T>, DispatchError> {
        ensure!(
            <Bounties<T>>::contains_key(bounty_id),
            Error::<T>::BountyDoesntExist
        );

        let bounty = <Bounties<T>>::get(bounty_id);

        Ok(bounty)
    }

    // Calculate cherry fraction to reward member for an unsuccessful bounty.
    // Cherry fraction = cherry * (member funding / total funding).
    fn get_cherry_fraction_for_member(
        bounty: &Bounty<T>,
        funding_amount: BalanceOf<T>,
    ) -> BalanceOf<T> {
        let funding_share =
            Perbill::from_rational_approximation(funding_amount, bounty.total_funding);

        // cherry share
        funding_share * bounty.creation_params.cherry
    }

    // Calculate cherry to withdraw by bounty creator.
    fn get_cherry_for_creator_withdrawal(bounty: &Bounty<T>, stage: BountyStage) -> BalanceOf<T> {
        if let BountyStage::Withdrawal {
            cherry_needs_withdrawal,
        } = stage
        {
            if cherry_needs_withdrawal {
                return bounty.creation_params.cherry;
            }
        }

        Zero::zero()
    }

    // Remove bounty and all related info from the storage.
    fn remove_bounty(bounty_id: &T::BountyId) {
        <Bounties<T>>::remove(bounty_id);
        <BountyContributions<T>>::remove_prefix(bounty_id);
        <WorkEntries<T>>::remove_prefix(bounty_id);

        // Slash remaining funds.
        let bounty_account_id = Self::bounty_account_id(*bounty_id);
        let all = balances::Module::<T>::usable_balance(&bounty_account_id);
        if all != Zero::zero() {
            let _ = balances::Module::<T>::slash(&bounty_account_id, all);
        }

        Self::deposit_event(RawEvent::BountyRemoved(*bounty_id));
    }

    // Verifies that the bounty has no pending fund withdrawals left.
    fn withdrawal_completed(stage: &BountyStage, bounty_id: &T::BountyId) -> bool {
        !Self::contributions_exist(bounty_id)
            && !Self::work_entries_exist(bounty_id)
            && matches!(
                stage,
                BountyStage::Withdrawal {
                    cherry_needs_withdrawal: false,
                }
            )
    }

    // Verifies that bounty has some contribution to withdraw.
    // Should be O(1) because of the single inner call of the next() function of the iterator.
    pub(crate) fn contributions_exist(bounty_id: &T::BountyId) -> bool {
        <BountyContributions<T>>::iter_prefix_values(bounty_id)
            .peekable()
            .peek()
            .is_some()
    }

    // Verifies that bounty has some work entries to withdraw.
    // Should be O(1) because of the single inner call of the next() function of the iterator.
    pub(crate) fn work_entries_exist(bounty_id: &T::BountyId) -> bool {
        <BountyContributions<T>>::iter_prefix_values(bounty_id)
            .peekable()
            .peek()
            .is_some()
    }

    // The account ID of a bounty account. Tests require AccountID type to be at least u128.
    pub(crate) fn bounty_account_id(bounty_id: T::BountyId) -> T::AccountId {
        T::ModuleId::get().into_sub_account(bounty_id)
    }

    // Calculates bounty milestone on member funding.
    fn get_bounty_milestone_on_funding(
        maximum_funding_reached: bool,
        previous_milestone: BountyMilestone<T::BlockNumber>,
    ) -> BountyMilestone<T::BlockNumber> {
        let now = Self::current_block();

        if maximum_funding_reached {
            // Bounty maximum funding reached.
            BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: now,
            }
        // No previous contributions.
        } else if let BountyMilestone::Created {
            created_at,
            has_contributions: false,
        } = previous_milestone
        {
            // The bounty has some contributions now.
            BountyMilestone::Created {
                created_at,
                has_contributions: true,
            }
        } else {
            // No changes.
            previous_milestone
        }
    }

    // Calculates bounty milestone on work submitting.
    fn get_bounty_milestone_on_work_submitting(
        bounty: &Bounty<T>,
    ) -> BountyMilestone<T::BlockNumber> {
        let previous_milestone = bounty.milestone.clone();

        match bounty.milestone.clone() {
            BountyMilestone::Created { created_at, .. } => {
                // Limited funding period.
                if let Some(funding_period) = bounty.creation_params.funding_period {
                    return BountyMilestone::WorkSubmitted {
                        work_period_started_at: created_at + funding_period,
                    };
                }

                // Unlimited funding period.
                previous_milestone
            }
            BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at,
            } => BountyMilestone::WorkSubmitted {
                work_period_started_at: max_funding_reached_at,
            },
            _ => previous_milestone,
        }
    }

    // Validates stake on announcing the work entry.
    fn validate_entrant_stake(
        member_id: MemberId<T>,
        bounty: &Bounty<T>,
        staking_account_id: Option<T::AccountId>,
    ) -> Result<Option<RequiredStakeInfo<T>>, DispatchError> {
        let staking_balance = bounty.creation_params.entrant_stake;

        if staking_balance != Zero::zero() {
            if let Some(staking_account_id) = staking_account_id {
                ensure!(
                    T::StakingAccountValidator::is_member_staking_account(
                        &member_id,
                        &staking_account_id
                    ),
                    Error::<T>::InvalidStakingAccountForMember
                );

                ensure!(
                    T::StakingHandler::is_account_free_of_conflicting_stakes(&staking_account_id),
                    Error::<T>::ConflictingStakes
                );

                ensure!(
                    T::StakingHandler::is_enough_balance_for_stake(
                        &staking_account_id,
                        staking_balance
                    ),
                    Error::<T>::InsufficientBalanceForStake
                );

                Ok(Some(RequiredStakeInfo {
                    amount: staking_balance,
                    account_id: staking_account_id,
                }))
            } else {
                // No staking account when required.
                Err(Error::<T>::NoStakingAccountProvided.into())
            }
        } else {
            // No stake required
            Ok(None)
        }
    }

    // Verifies work entry existence and retrieves an entry from the storage.
    fn ensure_work_entry_exists(
        bounty_id: &T::BountyId,
        entry_id: &T::WorkEntryId,
    ) -> Result<WorkEntry<T>, DispatchError> {
        ensure!(
            <WorkEntries<T>>::contains_key(bounty_id, entry_id),
            Error::<T>::WorkEntryDoesntExist
        );

        let entry = <WorkEntries<T>>::get(bounty_id, entry_id);

        Ok(entry)
    }

    // Unlocks the work entry stake.
    // It also calculates and slashes the stake on work entry withdrawal.
    // The slashing amount depends on the entry active period.
    fn unlock_work_entry_stake(bounty: &Bounty<T>, entry: &WorkEntry<T>) {
        if let Some(staking_account_id) = &entry.staking_account_id {
            let now = Self::current_block();
            let staking_balance = bounty.creation_params.entrant_stake;

            let entry_was_active_period = now.saturating_sub(entry.submitted_at);

            let slashing_share = Perbill::from_rational_approximation(
                entry_was_active_period,
                bounty.creation_params.work_period,
            );

            // No more than staking_balance.
            let slashing_amount = (slashing_share * staking_balance).min(staking_balance);

            if slashing_amount > Zero::zero() {
                T::StakingHandler::slash(staking_account_id, Some(slashing_amount));
            }

            T::StakingHandler::unlock(staking_account_id);
        }
    }

    // Slashed the work entry stake.
    fn slash_work_entry_stake(entry: &WorkEntry<T>) {
        if let Some(staking_account_id) = &entry.staking_account_id {
            T::StakingHandler::slash(staking_account_id, None);
        }
    }

    // Validates the contract type for a bounty
    fn ensure_valid_contract_type(bounty: &Bounty<T>, member_id: &MemberId<T>) -> DispatchResult {
        if let AssuranceContractType::Closed(ref valid_members) =
            bounty.creation_params.contract_type
        {
            ensure!(
                valid_members.contains(member_id),
                Error::<T>::CannotSubmitWorkToClosedContractBounty
            );
        }

        Ok(())
    }

    // Computes the stage of a bounty based on its creation parameters and the current state.
    pub(crate) fn get_bounty_stage(bounty: &Bounty<T>) -> BountyStage {
        let sc = BountyStageCalculator::<T> {
            now: Self::current_block(),
            bounty,
        };

        sc.is_funding_stage()
            .or_else(|| sc.is_work_submission_stage())
            .or_else(|| sc.is_judgment_stage())
            .unwrap_or_else(|| sc.withdrawal_stage())
    }

    // Validates oracle judgment.
    fn validate_judgment(
        bounty_id: &T::BountyId,
        judgment: &OracleJudgmentOf<T>,
    ) -> DispatchResult {
        // Check work entry existence.
        for (entry_id, _) in judgment.iter() {
            ensure!(
                <WorkEntries<T>>::contains_key(bounty_id, entry_id),
                Error::<T>::WorkEntryDoesntExist
            );
        }

        //TODO: validate judgement winner ratio
        //TODO: check non-zero winner ratio

        Ok(())
    }

    // Removes the work entry and decrements active entry count in a bounty.
    fn remove_work_entry(bounty_id: &T::BountyId, entry_id: &T::WorkEntryId) {
        <WorkEntries<T>>::remove(bounty_id, entry_id);

        // Decrement work entry counter and update bounty record.
        <Bounties<T>>::mutate(bounty_id, |bounty| {
            bounty.decrement_active_work_entry_counter();
        });
    }

    // Calculates weight for submit_oracle_judgement extrinsic.
    fn submit_oracle_judgement_weight(judgement: &OracleJudgmentOf<T>) -> Weight {
        let collection_length: u32 = judgement.len().saturated_into();

        WeightInfoBounty::<T>::submit_oracle_judgment_by_council_all_winners(collection_length)
            .max(
                WeightInfoBounty::<T>::submit_oracle_judgment_by_council_all_rejected(
                    collection_length,
                ),
            )
            .max(
                WeightInfoBounty::<T>::submit_oracle_judgment_by_member_all_winners(
                    collection_length,
                ),
            )
            .max(
                WeightInfoBounty::<T>::submit_oracle_judgment_by_member_all_rejected(
                    collection_length,
                ),
            )
    }

    // Bounty stage validator.
    fn ensure_bounty_stage(
        actual_stage: BountyStage,
        expected_stage: BountyStage,
    ) -> DispatchResult {
        ensure!(
            actual_stage == expected_stage,
            Self::unexpected_bounty_stage_error(actual_stage)
        );

        Ok(())
    }

    // Bounty withdrawal stage validator. Returns `cherry_needs_withdrawal` flag.
    fn ensure_bounty_withdrawal_stage(actual_stage: BountyStage) -> Result<bool, DispatchError> {
        if let BountyStage::Withdrawal {
            cherry_needs_withdrawal,
        } = actual_stage
        {
            Ok(cherry_needs_withdrawal)
        } else {
            Err(Self::unexpected_bounty_stage_error(actual_stage))
        }
    }

    // Provides fined-grained errors for a bounty stages
    fn unexpected_bounty_stage_error(unexpected_stage: BountyStage) -> DispatchError {
        match unexpected_stage {
            BountyStage::Funding { .. } => Error::<T>::InvalidStageUnexpectedFunding.into(),
            BountyStage::WorkSubmission => Error::<T>::InvalidStageUnexpectedWorkSubmission.into(),
            BountyStage::Judgment => Error::<T>::InvalidStageUnexpectedJudgment.into(),
            BountyStage::Withdrawal { .. } => Error::<T>::InvalidStageUnexpectedWithdrawal.into(),
        }
    }

    // Oracle judgment helper. Returns true if a judgement contains at least one winner.
    pub(crate) fn judgment_has_winners(judgment: &OracleJudgmentOf<T>) -> bool {
        judgment.iter().any(|(_, j)| j.is_winner())
    }
}
