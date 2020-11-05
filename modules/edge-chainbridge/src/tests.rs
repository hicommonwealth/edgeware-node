#![cfg(test)]

use super::mock::{
    assert_events, balances, event_exists, expect_event, new_test_ext, Balances, Bridge, Call,
    Event, Example, NativeTokenId, Origin, ProposalLifetime, Test,
    ENDOWED_BALANCE, RELAYER_A, RELAYER_B, RELAYER_C,
};
use super::*;
use frame_support::dispatch::DispatchError;
use frame_support::{assert_noop, assert_ok};

use codec::Encode;
use sp_core::{blake2_256, H256};

const TEST_THRESHOLD: u32 = 2;


fn make_transfer_proposal(to: u64, amount: u64) -> Call {
    Call::Example(crate::Call::transfer(to, amount.into()))
}

#[test]
fn transfer_native() {
    new_test_ext().execute_with(|| {
        let dest_chain = 0;
        let resource_id = NativeTokenId::get();
        let amount: u64 = 100;
        let recipient = vec![99];

        assert_ok!(Bridge::whitelist_chain(Origin::root(), dest_chain.clone()));
        assert_ok!(Example::transfer_native(
            Origin::signed(RELAYER_A),
            amount.clone(),
            recipient.clone(),
            dest_chain,
        ));

        expect_event(bridge::RawEvent::FungibleTransfer(
            dest_chain,
            1,
            resource_id,
            amount.into(),
            recipient,
        ));
    })
}

#[test]
fn transfer() {
    new_test_ext().execute_with(|| {
        // Check inital state
        let bridge_id: u64 = Bridge::account_id();
        assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE);
        // Transfer and check result
        assert_ok!(Example::transfer(
            Origin::signed(Bridge::account_id()),
            RELAYER_A,
            10
        ));
        assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE - 10);
        assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

        assert_events(vec![Event::balances(balances::RawEvent::Transfer(
            Bridge::account_id(),
            RELAYER_A,
            10,
        ))]);
    })
}

#[test]
fn create_sucessful_transfer_proposal() {
    new_test_ext().execute_with(|| {
        let prop_id = 1;
        let src_id = 1;
        let r_id = bridge::derive_resource_id(src_id, b"transfer");
        let resource = b"Example.transfer".to_vec();
        let proposal = make_transfer_proposal(RELAYER_A, 10);

        assert_ok!(Bridge::set_threshold(Origin::root(), TEST_THRESHOLD,));
        assert_ok!(Bridge::add_relayer(Origin::root(), RELAYER_A));
        assert_ok!(Bridge::add_relayer(Origin::root(), RELAYER_B));
        assert_ok!(Bridge::add_relayer(Origin::root(), RELAYER_C));
        assert_ok!(Bridge::whitelist_chain(Origin::root(), src_id));
        assert_ok!(Bridge::set_resource(Origin::root(), r_id, resource));

        // Create proposal (& vote)
        assert_ok!(Bridge::acknowledge_proposal(
            Origin::signed(RELAYER_A),
            prop_id,
            src_id,
            r_id,
            Box::new(proposal.clone())
        ));
        let prop = Bridge::votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
        let expected = bridge::ProposalVotes {
            votes_for: vec![RELAYER_A],
            votes_against: vec![],
            status: bridge::ProposalStatus::Initiated,
            expiry: ProposalLifetime::get() + 1,
        };
        assert_eq!(prop, expected);

        // Second relayer votes against
        assert_ok!(Bridge::reject_proposal(
            Origin::signed(RELAYER_B),
            prop_id,
            src_id,
            r_id,
            Box::new(proposal.clone())
        ));
        let prop = Bridge::votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
        let expected = bridge::ProposalVotes {
            votes_for: vec![RELAYER_A],
            votes_against: vec![RELAYER_B],
            status: bridge::ProposalStatus::Initiated,
            expiry: ProposalLifetime::get() + 1,
        };
        assert_eq!(prop, expected);

        // Third relayer votes in favour
        assert_ok!(Bridge::acknowledge_proposal(
            Origin::signed(RELAYER_C),
            prop_id,
            src_id,
            r_id,
            Box::new(proposal.clone())
        ));
        let prop = Bridge::votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
        let expected = bridge::ProposalVotes {
            votes_for: vec![RELAYER_A, RELAYER_C],
            votes_against: vec![RELAYER_B],
            status: bridge::ProposalStatus::Approved,
            expiry: ProposalLifetime::get() + 1,
        };
        assert_eq!(prop, expected);

        assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);
        assert_eq!(
            Balances::free_balance(Bridge::account_id()),
            ENDOWED_BALANCE - 10
        );

        assert_events(vec![
            Event::bridge(bridge::RawEvent::VoteFor(src_id, prop_id, RELAYER_A)),
            Event::bridge(bridge::RawEvent::VoteAgainst(src_id, prop_id, RELAYER_B)),
            Event::bridge(bridge::RawEvent::VoteFor(src_id, prop_id, RELAYER_C)),
            Event::bridge(bridge::RawEvent::ProposalApproved(src_id, prop_id)),
            Event::balances(balances::RawEvent::Transfer(
                Bridge::account_id(),
                RELAYER_A,
                10,
            )),
            Event::bridge(bridge::RawEvent::ProposalSucceeded(src_id, prop_id)),
        ]);
    })
}
