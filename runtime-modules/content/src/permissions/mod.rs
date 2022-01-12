// The following table summarizes the permissions in the content subsystem.
// - Actor role as columns, controller account is Tx sender.
// - operations on a given channel (=channel=) are rows, which are basically the guards to be
//   implemented
// - Entries are conditions to be verified / assertions
//
// |                       | *Lead*                   | *Curator*                | *Member*                | *Collaborator*            |
// |-----------------------+--------------------------+--------------------------+-------------------------+---------------------------|
// | *assets mgmt*         | channel.owner is curator | curator is channel.owner | member is channel.owner | collab in channel.collabs |
// | *censorship mgmt*     | true                     | channel.owner is member  | false                   | false                     |
// | *category mgmt*       | true                     | true                     | false                   | false                     |
// | *collab. set mgmt*    | channel.owner is curator | curator is channel.owner | member is channel.owner | false                     |
// | *reward account mgmt* | channel.owner is curator | curator is channel.owner | member is channel.owner | false                     |
// | *create channel*      | false                    | true                     | true                    | false                     |
// | *delete channel*      | channel.owner is curator | curator is channel.owner | member is channel.owner | false                     |

mod curator_group;

pub use curator_group::*;

pub use crate::errors::*;
use crate::*;
pub use codec::{Codec, Decode, Encode};
use core::fmt::Debug;
use frame_support::{ensure, Parameter};
#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::traits::{MaybeSerializeDeserialize, Member};

/// Model of authentication manager.
pub trait ContentActorAuthenticator: frame_system::Trait + membership::Trait {
    /// Curator identifier
    type CuratorId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord;

    /// Curator group identifier
    type CuratorGroupId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord;

    /// Authorize actor as lead
    fn is_lead(account_id: &Self::AccountId) -> bool;

    /// Checks if Id represents a worker id in the working group
    fn is_valid_curator_id(curator_id: &Self::CuratorId) -> bool;

    /// Authorize actor as curator
    fn is_curator(curator_id: &Self::CuratorId, account_id: &Self::AccountId) -> bool;

    /// Authorize actor as member
    fn is_member(member_id: &Self::MemberId, account_id: &Self::AccountId) -> bool;

    /// Ensure member id is valid
    fn validate_member_id(member_id: &Self::MemberId) -> bool;
}

pub fn ensure_is_valid_curator_id<T: Trait>(curator_id: &T::CuratorId) -> DispatchResult {
    ensure!(
        T::is_valid_curator_id(curator_id),
        Error::<T>::CuratorIdInvalid
    );
    Ok(())
}

/// Ensure curator authorization performed succesfully
pub fn ensure_curator_auth_success<T: Trait>(
    curator_id: &T::CuratorId,
    account_id: &T::AccountId,
) -> DispatchResult {
    ensure!(
        T::is_curator(curator_id, account_id),
        Error::<T>::CuratorAuthFailed
    );
    Ok(())
}

/// Ensure member authorization performed succesfully
pub fn ensure_member_auth_success<T: Trait>(
    account_id: &T::AccountId,
    member_id: &T::MemberId,
) -> DispatchResult {
    ensure!(
        T::is_member(member_id, account_id),
        Error::<T>::MemberAuthFailed
    );
    Ok(())
}

/// Ensure lead authorization performed succesfully
pub fn ensure_lead_auth_success<T: Trait>(account_id: &T::AccountId) -> DispatchResult {
    ensure!(T::is_lead(account_id), Error::<T>::LeadAuthFailed);
    Ok(())
}

/// Ensure actor is authorized to create a channel
pub fn ensure_actor_authorized_to_create_channel<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    match actor {
        ContentActor::Curator(curator_group_id, curator_id) => {
            // Authorize curator, performing all checks to ensure curator can act
            CuratorGroup::<T>::perform_curator_in_group_auth(curator_id, curator_group_id, &sender)
        }
        ContentActor::Member(member_id) => ensure_member_auth_success::<T>(sender, member_id),
        // Lead & collaborators should use their member or curator role to create channels.
        _ => Err(Error::<T>::ActorCannotOwnChannel.into()),
    }
}

/// Ensure actor is authorized to delete channel
pub fn ensure_actor_authorized_to_delete_channel<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    channel_owner: &ChannelOwner<T::MemberId, T::CuratorGroupId>,
) -> DispatchResult {
    match actor {
        ContentActor::Lead => {
            // ensure lead is valid
            ensure_lead_auth_success::<T>(sender)?;
            // ensure curator
            ensure_channel_is_owned_by_curators::<T>(channel_owner)?;
            Ok(())
        }
        ContentActor::Curator(curator_group_id, curator_id) => {
            // ensure curator group is valid
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )?;
            // ensure group is channel owner
            ensure_curator_group_is_channel_owner::<T>(channel_owner, curator_group_id)?;
            Ok(())
        }
        ContentActor::Member(member_id) => {
            // ensure valid member
            ensure_member_auth_success::<T>(&sender, member_id)?;
            // ensure member is channel owner
            ensure_member_is_channel_owner::<T>(channel_owner, member_id)?;
            Ok(())
        }
        // collaborators should use their member or curator role to delete channel
        _ => Err(Error::<T>::ActorNotAuthorized.into()),
    }
}

/// Ensure actor is authorized to manage collaborator set for a channel
pub fn ensure_actor_can_manage_collaborators<T: Trait>(
    sender: &T::AccountId,
    channel_owner: &ChannelOwner<T::MemberId, T::CuratorGroupId>,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    match actor {
        ContentActor::Lead => {
            // ensure lead is valid
            ensure_lead_auth_success::<T>(sender)?;
            // ensure curator
            ensure_channel_is_owned_by_curators::<T>(channel_owner)?;
            Ok(())
        }
        ContentActor::Curator(curator_group_id, curator_id) => {
            // ensure curator group is valid
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )?;
            // ensure group is channel owner
            ensure_curator_group_is_channel_owner::<T>(channel_owner, curator_group_id)?;
            Ok(())
        }
        ContentActor::Member(member_id) => {
            // ensure valid member
            ensure_member_auth_success::<T>(&sender, member_id)?;
            // ensure member is channel owner
            ensure_member_is_channel_owner::<T>(channel_owner, member_id)?;
            Ok(())
        }
        // Collaborators should use their member or curator role in order to update reward account.
        ContentActor::Collaborator(_) => Err(Error::<T>::ActorNotAuthorized.into()),
    }
}

/// Ensure actor is authorized to manage reward account for a channel
pub fn ensure_actor_can_manage_reward_account<T: Trait>(
    sender: &T::AccountId,
    channel_owner: &ChannelOwner<T::MemberId, T::CuratorGroupId>,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    match actor {
        ContentActor::Lead => {
            // ensure lead is valid
            ensure_lead_auth_success::<T>(sender)?;
            // ensure curator
            ensure_channel_is_owned_by_curators::<T>(channel_owner)?;
            Ok(())
        }
        ContentActor::Curator(curator_group_id, curator_id) => {
            // ensure curator group is valid
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )?;
            // ensure group is channel owner
            ensure_curator_group_is_channel_owner::<T>(channel_owner, curator_group_id)?;
            Ok(())
        }
        ContentActor::Member(member_id) => {
            // ensure valid member
            ensure_member_auth_success::<T>(&sender, member_id)?;
            // ensure member is channel owner
            ensure_member_is_channel_owner::<T>(channel_owner, member_id)?;
            Ok(())
        }
        // collaborators should use their member or curator role in order to update reward account.
        _ => Err(Error::<T>::ActorNotAuthorized.into()),
    }
}

/// Ensure actor is authorized to manage channel assets, video also qualify as assets
pub fn ensure_actor_authorized_to_update_channel_assets<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    channel: &Channel<T>,
) -> DispatchResult {
    // Only owner of a channel can update and delete channel assets.
    // Lead can update and delete curator group owned channel assets.
    match actor {
        ContentActor::Lead => {
            // ensure lead is valid
            ensure_lead_auth_success::<T>(sender)?;
            // ensure curator
            ensure_channel_is_owned_by_curators::<T>(&channel.owner)?;
            Ok(())
        }
        ContentActor::Curator(curator_group_id, curator_id) => {
            // ensure curator group is valid
            CuratorGroup::<T>::perform_curator_in_group_auth(curator_id, curator_group_id, sender)?;
            // ensure group is channel owner
            ensure_curator_group_is_channel_owner::<T>(&channel.owner, curator_group_id)?;
            Ok(())
        }
        ContentActor::Member(member_id) => {
            // ensure member account and origin correspondence
            ensure_member_auth_success::<T>(sender, member_id)?;
            // ensure member is channel owner
            ensure_member_is_channel_owner::<T>(&channel.owner, member_id)?;
            Ok(())
        }
        ContentActor::Collaborator(member_id) => {
            // ensure member account and origin correspondence
            ensure_member_auth_success::<T>(sender, member_id)?;
            // ensure valid collaborator
            ensure!(
                channel.collaborators.contains(member_id),
                Error::<T>::ActorNotAuthorized
            );
            Ok(())
        }
    }
}

// Enure actor can update channels and videos in the channel
pub fn ensure_actor_is_channel_owner<T: Trait>(
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    owner: &ChannelOwner<T::MemberId, T::CuratorGroupId, T::DAOId>,
) -> DispatchResult {
    let resulting_owner = actor_to_channel_owner::<T>(actor)?;
    ensure!(resulting_owner == *owner, Error::<T>::ActorNotAuthorized);
    Ok(())
}

// Enure actor can update or delete channels and videos
pub fn ensure_actor_authorized_to_set_featured_videos<T: Trait>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    // Only Lead authorized to set featured videos
    if let ContentActor::Lead = actor {
        let sender = ensure_signed(origin)?;
        ensure_lead_auth_success::<T>(&sender)
    } else {
        Err(Error::<T>::ActorNotAuthorized.into())
    }
}

/// Ensure actor can censor
pub fn ensure_actor_authorized_to_censor<T: Trait>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    owner: &ChannelOwner<T::MemberId, T::CuratorGroupId>,
) -> DispatchResult {
    // Only lead and curators can censor channels and videos
    // Only lead can censor curator group owned channels and videos
    let sender = ensure_signed(origin)?;
    ensure_actor_auth_success::<T>(&sender, actor)?;
    match actor {
        ContentActor::Lead => Ok(()),
        ContentActor::Curator(..) => {
            // Curators cannot censor curator group channels
            if let ChannelOwner::CuratorGroup(_) = owner {
                Err(Error::<T>::CannotCensoreCuratorGroupOwnedChannels.into())
            } else {
                Ok(())
            }
        }
        _ => {
            // Members & collaborators cannot censore channels!
            Err(Error::<T>::ActorNotAuthorized.into())
        }
    }
}

/// Ensure actor can manage categories
pub fn ensure_actor_authorized_to_manage_categories<T: Trait>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    // Only lead and curators can manage categories
    let sender = ensure_signed(origin)?;
    ensure_actor_auth_success::<T>(&sender, actor)?;
    match actor {
        ContentActor::Lead => {
            let sender = ensure_signed(origin)?;
            ensure_lead_auth_success::<T>(&sender)
        }
        ContentActor::CuratorGroup(curator_group_id, curator_id) => {
            // Authorize curator, performing all checks to ensure curator can act
            CuratorGroup::<T>::perform_curator_in_group_auth(curator_id, curator_group_id, &sender)
        }
        _ => {
            // Members & collaborators cannot manage categories!
            Err(Error::<T>::ActorNotAuthorized.into())
        }
    }
}

// authenticate actor
pub fn ensure_actor_auth_success<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    match actor {
        ContentActor::Lead => ensure_lead_auth_success::<T>(sender),
        ContentActor::Curator(curator_group_id, curator_id) => {
            CuratorGroup::<T>::perform_curator_in_group_auth(curator_id, curator_group_id, &sender)
        }
        ContentActor::Member(member_id) => ensure_member_auth_success::<T>(member_id, sender),
    }
}

// POST RELATED PERMISSIONS

// Enure actor can add a comment
pub fn ensure_actor_authorized_to_add_comment<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    ensure_actor_auth_success::<T>(sender, actor)
}

// Ensure actor can add a video post
pub fn ensure_actor_authorized_to_add_video_post<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    owner: &ChannelOwner<T::MemberId, T::CuratorGroupId, T::DAOId>,
) -> DispatchResult {
    ensure_actor_auth_success::<T>(sender, actor)?;
    ensure_actor_is_channel_owner::<T>(actor, owner)
}

// Enure actor can edit a video post description
pub fn ensure_actor_authorized_to_edit_video_post<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    owner: &ChannelOwner<T::MemberId, T::CuratorGroupId, T::DAOId>,
) -> DispatchResult {
    ensure_actor_auth_success::<T>(sender, actor)?;
    ensure_actor_is_channel_owner::<T>(actor, owner)
}

// Enure actor can edit a post comment text
pub fn ensure_actor_authorized_to_edit_comment<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    post: &Post<T>,
) -> DispatchResult {
    ensure_actor_auth_success::<T>(sender, actor)?;
    ensure_actor_is_comment_author::<T>(actor, &post.author)
}

// Enure actor can create post: same rules as if he is trying to update channel
pub fn ensure_actor_authorized_to_remove_comment<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    channel: &Channel<T>,
    post: &Post<T>,
) -> Result<CleanupActor, DispatchError> {
    ensure_actor_auth_success::<T>(sender, actor)?;
    let actor_is_owner = ensure_actor_is_channel_owner::<T>(actor, &channel.owner)
        .map(|_| CleanupActor::ChannelOwner);
    let actor_is_author =
        ensure_actor_is_comment_author::<T>(actor, &post.author).map(|_| CleanupActor::PostAuthor);
    let actor_is_moderator = ensure_actor_is_moderator::<T>(actor, &channel.moderator_set)
        .map(|_| CleanupActor::Moderator);

    actor_is_author.or(actor_is_owner).or(actor_is_moderator)
}

// Enure actor can create post: same rules as if he is trying to update channel
pub fn ensure_actor_authorized_to_remove_video_post<T: Trait>(
    sender: &T::AccountId,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    channel: &Channel<T>,
) -> DispatchResult {
    ensure_actor_auth_success::<T>(sender, actor)?;
    ensure_actor_is_channel_owner::<T>(actor, &channel.owner)
}

// Ensure actor is a moderator
pub fn ensure_actor_is_moderator<T: Trait>(
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    moderators: &BTreeSet<T::MemberId>,
) -> DispatchResult {
    match actor {
        ContentActor::Member(member_id) => {
            ensure!(
                moderators.contains(member_id),
                Error::<T>::ActorNotAuthorized
            );
            Ok(())
        }
        _ => Err(Error::<T>::ActorNotAuthorized.into()),
    }
}
// Enure actor is comment author
pub fn ensure_actor_is_comment_author<T: Trait>(
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    author: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    // authenticate actor
    ensure!(actor == author, Error::<T>::ActorNotAuthorized);
    Ok(())
}

pub fn actor_to_channel_owner<T: Trait>(
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> ActorToChannelOwnerResult<T> {
    match actor {
            // Lead should use their member or curator role to create channels
            ContentActor::Lead => Err(Error::<T>::ActorCannotOwnChannel),
            ContentActor::Curator(
                curator_group_id,
                _curator_id
            ) => {
                Ok(ChannelOwner::CuratorGroup(*curator_group_id))
            }
            ContentActor::Member(member_id) => {
                Ok(ChannelOwner::Member(*member_id))
            }
            // TODO:
            // ContentActor::Dao(id) => Ok(ChannelOwner::Dao(id)),
        }
}

/// Enum, representing all possible `Actor`s
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ContentActor<
    CuratorGroupId: Default + Clone + Copy,
    CuratorId: Default + Clone + Copy,
    MemberId: Default + Clone + Copy,
> {
    Curator(CuratorGroupId, CuratorId),
    Member(MemberId),
    Lead,
    Collaborator(MemberId),
}

impl<
        CuratorGroupId: Default + Clone + Copy,
        CuratorId: Default + Clone + Copy,
        MemberId: Default + Clone + Copy,
    > Default for ContentActor<CuratorGroupId, CuratorId, MemberId>
{
    fn default() -> Self {
        Self::Lead
    }
}
