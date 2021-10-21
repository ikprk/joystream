#![cfg(test)]

use crate::tests::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok};

#[test]
fn cancel_open_auction_bid() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        let video_id = NextVideoId::<Test>::get();

        create_simple_channel_and_video(FIRST_MEMBER_ORIGIN, FIRST_MEMBER_ID);

        // Issue nft
        assert_ok!(Content::issue_nft(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            video_id,
            None,
            b"metablob".to_vec(),
            None
        ));

        let bid_lock_duration = Content::min_bid_lock_duration();

        let auction_params = AuctionParams {
            starting_price: Content::min_starting_price(),
            buy_now_price: None,
            auction_type: AuctionType::Open(OpenAuctionDetails {
                bid_lock_duration,
            }),
            minimal_bid_step: Content::min_bid_step(),
            starts_at: None,
            whitelist: BTreeSet::new(),
        };

        // Start nft auction
        assert_ok!(Content::start_nft_auction(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            video_id,
            auction_params.clone(),
        ));

        // deposit initial balance
        let bid = Content::min_starting_price();

        let _ = balances::Module::<Test>::deposit_creating(&SECOND_MEMBER_ORIGIN, bid);

        // Make nft auction bid
        assert_ok!(Content::make_bid(
            Origin::signed(SECOND_MEMBER_ORIGIN),
            SECOND_MEMBER_ID,
            video_id,
            bid,
            vec![],
        ));

        // Runtime tested state before call

        // Events number before tested calls
        let number_of_events_before_call = System::events().len();

        // Run to the block where bid lock duration expires
        run_to_block(bid_lock_duration + 1);

        // Cancel auction bid
        assert_ok!(Content::cancel_open_auction_bid(
            Origin::signed(SECOND_MEMBER_ORIGIN),
            SECOND_MEMBER_ID,
            video_id,
        ));

        // Runtime tested state after call

        // Ensure bid on specific auction successfully canceled
        assert!(matches!(
            Content::video_by_id(video_id).nft_status,
            Some(OwnedNFT {
                transactional_status: TransactionalStatus::Auction(auction_without_bid,),
                ..
            }) if auction_without_bid.last_bid.is_none()
        ));

        let cancel_open_auction_bid_event = get_test_event(RawEvent::AuctionBidCanceled(
            SECOND_MEMBER_ID,
            video_id,
        ));

        // Last event checked
        assert_event(cancel_open_auction_bid_event, number_of_events_before_call + 1);
    })
}