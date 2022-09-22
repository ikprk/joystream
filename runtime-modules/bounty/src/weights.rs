// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bounty
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --pallet=bounty
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/bounty/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for bounty.
pub trait WeightInfo {
	fn create_bounty_by_council(_i: u32, _j: u32, ) -> Weight;
	fn create_bounty_by_member(_i: u32, _j: u32, ) -> Weight;
	fn terminate_bounty_w_oracle_reward_funding_expired() -> Weight;
	fn terminate_bounty_wo_oracle_reward_funding_expired() -> Weight;
	fn terminate_bounty_w_oracle_reward_wo_funds_funding() -> Weight;
	fn terminate_bounty_wo_oracle_reward_wo_funds_funding() -> Weight;
	fn terminate_bounty_w_oracle_reward_w_funds_funding() -> Weight;
	fn terminate_bounty_wo_oracle_reward_w_funds_funding() -> Weight;
	fn terminate_bounty_work_or_judging_period() -> Weight;
	fn fund_bounty_by_member() -> Weight;
	fn fund_bounty_by_council() -> Weight;
	fn withdraw_funding_by_member() -> Weight;
	fn withdraw_funding_by_council() -> Weight;
	fn announce_work_entry(_i: u32, _j: u32, ) -> Weight;
	fn submit_work(_i: u32, ) -> Weight;
	fn submit_oracle_judgment_by_council(_j: u32, _k: u32, _w: u32, _r: u32, ) -> Weight;
	fn submit_oracle_judgment_by_member(_j: u32, _k: u32, _w: u32, _r: u32, ) -> Weight;
	fn switch_oracle_to_council_by_council_successful() -> Weight;
	fn switch_oracle_to_member_by_oracle_council() -> Weight;
	fn switch_oracle_to_member_by_council() -> Weight;
	fn switch_oracle_to_member_by_oracle_member() -> Weight;
	fn switch_oracle_to_council_by_oracle_member() -> Weight;
	fn end_working_period() -> Weight;
	fn withdraw_entrant_stake() -> Weight;
	fn withdraw_funding_state_bloat_bond_by_council() -> Weight;
	fn withdraw_funding_state_bloat_bond_by_member() -> Weight;
	fn withdraw_oracle_reward_by_oracle_council() -> Weight;
	fn withdraw_oracle_reward_by_oracle_member() -> Weight;
	fn entrant_remark(_i: u32, ) -> Weight;
	fn contributor_remark(_i: u32, ) -> Weight;
	fn oracle_remark(_i: u32, ) -> Weight;
	fn creator_remark(_i: u32, ) -> Weight;
}

/// Weights for bounty using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Membership MembershipById (r:50 w:0)
	// Storage: Council Budget (r:1 w:1)
	// Storage: Bounty BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounty Bounties (r:0 w:1)
	fn create_bounty_by_council(i: u32, j: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 113_000
			.saturating_add((7_204_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:51 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounty BountyCount (r:1 w:1)
	// Storage: Bounty Bounties (r:0 w:1)
	fn create_bounty_by_member(i: u32, j: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 120_000
			.saturating_add((8_124_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn terminate_bounty_w_oracle_reward_funding_expired() -> Weight {
		(55_838_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn terminate_bounty_wo_oracle_reward_funding_expired() -> Weight {
		(78_070_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn terminate_bounty_w_oracle_reward_wo_funds_funding() -> Weight {
		(55_738_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn terminate_bounty_wo_oracle_reward_wo_funds_funding() -> Weight {
		(77_713_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:2 w:0)
	fn terminate_bounty_w_oracle_reward_w_funds_funding() -> Weight {
		(31_898_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:2 w:0)
	fn terminate_bounty_wo_oracle_reward_w_funds_funding() -> Weight {
		(38_004_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:2 w:0)
	fn terminate_bounty_work_or_judging_period() -> Weight {
		(31_508_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn fund_bounty_by_member() -> Weight {
		(63_961_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn fund_bounty_by_council() -> Weight {
		(53_867_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Bounties (r:1 w:0)
	// Storage: Bounty BountyContributions (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn withdraw_funding_by_member() -> Weight {
		(62_272_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:0)
	// Storage: Bounty BountyContributions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn withdraw_funding_by_council() -> Weight {
		(54_369_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounty EntryCount (r:1 w:1)
	// Storage: Bounty Entries (r:0 w:1)
	fn announce_work_entry(i: u32, j: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 101_000
			.saturating_add((2_571_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Bounties (r:1 w:0)
	// Storage: Bounty Entries (r:1 w:1)
	fn submit_work(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty Entries (r:150 w:150)
	// Storage: Membership MembershipById (r:150 w:0)
	// Storage: System Account (r:151 w:151)
	// Storage: Council Budget (r:1 w:1)
	// Storage: Balances Locks (r:150 w:150)
	fn submit_oracle_judgment_by_council(j: u32, k: u32, w: u32, r: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(k as Weight))
			// Standard Error: 318_000
			.saturating_add((61_007_000 as Weight).saturating_mul(w as Weight))
			// Standard Error: 318_000
			.saturating_add((80_959_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(w as Weight)))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(w as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:150 w:0)
	// Storage: Bounty Entries (r:149 w:149)
	// Storage: System Account (r:150 w:150)
	// Storage: Council Budget (r:1 w:1)
	// Storage: Balances Locks (r:149 w:149)
	fn submit_oracle_judgment_by_member(j: u32, k: u32, w: u32, r: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(k as Weight))
			// Standard Error: 337_000
			.saturating_add((57_565_000 as Weight).saturating_mul(w as Weight))
			// Standard Error: 343_000
			.saturating_add((83_084_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(w as Weight)))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(w as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	fn switch_oracle_to_council_by_council_successful() -> Weight {
		(24_948_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	fn switch_oracle_to_member_by_oracle_council() -> Weight {
		(28_007_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	fn switch_oracle_to_member_by_council() -> Weight {
		(29_984_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:2 w:0)
	fn switch_oracle_to_member_by_oracle_member() -> Weight {
		(33_274_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	fn switch_oracle_to_council_by_oracle_member() -> Weight {
		(28_794_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	fn end_working_period() -> Weight {
		(30_115_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Bounty Entries (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn withdraw_entrant_stake() -> Weight {
		(52_949_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:0)
	// Storage: Bounty BountyContributions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn withdraw_funding_state_bloat_bond_by_council() -> Weight {
		(54_476_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Bounties (r:1 w:0)
	// Storage: Bounty BountyContributions (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn withdraw_funding_state_bloat_bond_by_member() -> Weight {
		(60_659_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	// Storage: Bounty BountyContributions (r:1 w:0)
	fn withdraw_oracle_reward_by_oracle_council() -> Weight {
		(80_363_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounty BountyContributions (r:1 w:0)
	// Storage: Council Budget (r:1 w:1)
	fn withdraw_oracle_reward_by_oracle_member() -> Weight {
		(92_611_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty Entries (r:1 w:0)
	fn entrant_remark(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Bounty BountyContributions (r:1 w:0)
	fn contributor_remark(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:0)
	fn oracle_remark(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Bounty Bounties (r:1 w:0)
	fn creator_remark(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_bounty_by_council(i: u32, j: u32, ) -> Weight {
		0
	}
	fn create_bounty_by_member(i: u32, j: u32, ) -> Weight {
		0
	}
	fn terminate_bounty_w_oracle_reward_funding_expired() -> Weight {
		0
	}
	fn terminate_bounty_wo_oracle_reward_funding_expired() -> Weight {
		0
	}
	fn terminate_bounty_w_oracle_reward_wo_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_wo_oracle_reward_wo_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_w_oracle_reward_w_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_wo_oracle_reward_w_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_work_or_judging_period() -> Weight {
		0
	}
	fn fund_bounty_by_member() -> Weight {
		0
	}
	fn fund_bounty_by_council() -> Weight {
		0
	}
	fn withdraw_funding_by_member() -> Weight {
		0
	}
	fn withdraw_funding_by_council() -> Weight {
		0
	}
	fn announce_work_entry(i: u32, j: u32, ) -> Weight {
		0
	}
	fn submit_work(i: u32, ) -> Weight {
		0
	}
	fn submit_oracle_judgment_by_council(j: u32, k: u32, w: u32, r: u32, ) -> Weight {
		0
	}
	fn submit_oracle_judgment_by_member(j: u32, k: u32, w: u32, r: u32, ) -> Weight {
		0
	}
	fn switch_oracle_to_council_by_council_successful() -> Weight {
		0
	}
	fn switch_oracle_to_member_by_oracle_council() -> Weight {
		0
	}
	fn switch_oracle_to_member_by_council() -> Weight {
		0
	}
	fn switch_oracle_to_member_by_oracle_member() -> Weight {
		0
	}
	fn switch_oracle_to_council_by_oracle_member() -> Weight {
		0
	}
	fn end_working_period() -> Weight {
		0
	}
	fn withdraw_entrant_stake() -> Weight {
		0
	}
	fn withdraw_funding_state_bloat_bond_by_council() -> Weight {
		0
	}
	fn withdraw_funding_state_bloat_bond_by_member() -> Weight {
		0
	}
	fn withdraw_oracle_reward_by_oracle_council() -> Weight {
		0
	}
	fn withdraw_oracle_reward_by_oracle_member() -> Weight {
		0
	}
	fn entrant_remark(i: u32, ) -> Weight {
		0
	}
	fn contributor_remark(i: u32, ) -> Weight {
		0
	}
	fn oracle_remark(i: u32, ) -> Weight {
		0
	}
	fn creator_remark(i: u32, ) -> Weight {
		0
	}
}
