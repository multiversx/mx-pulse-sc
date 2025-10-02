mod merkle_proof_setup;
use merkle_proof_setup::MerkleProofs;
use std::vec;

use multiversx_sc_snippets::{hex, imports::*};
use rust_interact::{Config, Interact};

use crate::merkle_proof_setup::MerkleProofSetup;

const ONE_HOUR: u64 = 60 * 60;
const USER_ERROR_CODE: u64 = 4;
const ROOT_HASH: &[u8; 64] = b"078bc8a05f5e62733ca27a4e0df5f5ff2d7327c9ab6c7f4766b6af12b5cc9183";

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_pulse_sc_cs() {
    let mut interactor = Interact::new(Config::chain_simulator_config()).await;

    let merkle_proofs = MerkleProofs::new();

    interactor.deploy().await;
    let root_hash = hex::decode(ROOT_HASH).unwrap();
    interactor
        .set_root_hash(ManagedByteArray::new_from_bytes(
            root_hash.as_slice().try_into().unwrap(),
        ))
        .await;
    interactor
        .new_poll(
            "What's your favourite fruit?",
            vec!["apple", "grape", "watermelon", "tomato"],
            ONE_HOUR,
            Some(ExpectError(
                USER_ERROR_CODE,
                &"Endpoint can only be called by admins",
            )),
        )
        .await;

    let allice = test_wallets::alice().to_address();

    interactor.add_admin(Bech32Address::from(&allice)).await;

    interactor
        .new_poll(
            "What's your favourite fruit?",
            vec!["apple", "grape", "watermelon", "tomato"],
            ONE_HOUR,
            None,
        )
        .await;

    interactor
        .vote_poll(
            Bech32Address::from(&allice),
            0,
            2,
            1000000000000000000,
            merkle_proofs.pairs[&allice].clone(),
            None,
        )
        .await;

    // Bob tries to cheat with a wrong voting power

    let bob = test_wallets::bob().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&bob),
            0,
            0,
            2000000000000000000,
            merkle_proofs.pairs[&bob].clone(),
            Some(ExpectError(USER_ERROR_CODE, &"Invalid voting power proof")),
        )
        .await;

    // Bob tries to vote on an inexisting poll

    let bob = test_wallets::bob().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&bob),
            1,
            0,
            1000000000000000000,
            merkle_proofs.pairs[&bob].clone(),
            Some(ExpectError(
                USER_ERROR_CODE,
                &"Poll index provided does not exist",
            )),
        )
        .await;

    // Bob tried to vote an inexisting option

    let bob = test_wallets::bob().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&bob),
            0,
            4,
            1000000000000000000,
            merkle_proofs.pairs[&bob].clone(),
            Some(ExpectError(
                USER_ERROR_CODE,
                &"Option index provided does not exist",
            )),
        )
        .await;

    // more correct votes

    interactor
        .vote_poll(
            Bech32Address::from(&bob),
            0,
            0,
            1000000000000000000,
            merkle_proofs.pairs[&bob].clone(),
            None,
        )
        .await;

    let carol = test_wallets::carol().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&carol),
            0,
            2,
            2000000000000000000,
            merkle_proofs.pairs[&carol].clone(),
            None,
        )
        .await;
    let dan = test_wallets::dan().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&dan),
            0,
            2,
            3000000000000000000,
            merkle_proofs.pairs[&dan].clone(),
            None,
        )
        .await;
    let eve = test_wallets::eve().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&eve),
            0,
            2,
            4000000000000000000,
            merkle_proofs.pairs[&eve].clone(),
            None,
        )
        .await;
    let grace = test_wallets::grace().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&grace),
            0,
            1,
            6000000000000000000,
            merkle_proofs.pairs[&grace].clone(),
            None,
        )
        .await;

    // Bob tried to vote again

    interactor
        .vote_poll(
            Bech32Address::from(&bob),
            0,
            0,
            1000000000000000000,
            merkle_proofs.pairs[&bob].clone(),
            Some(ExpectError(
                USER_ERROR_CODE,
                &"You have already voted in this poll",
            )),
        )
        .await;

    interactor
        .end_poll(
            0,
            Some(ExpectError(USER_ERROR_CODE, &"Poll is still active")),
        )
        .await;

    interactor.generate_blocks_until_epoch(30).await;

    interactor.end_poll(0, None).await;

    // Frank tries to vote after the poll has ended

    let frank = test_wallets::frank().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&frank),
            0,
            3,
            5000000000000000000,
            merkle_proofs.pairs[&frank].clone(),
            Some(ExpectError(USER_ERROR_CODE, &"Poll has already ended")),
        )
        .await;

    // Trying to end the poll again from the inter

    interactor
        .end_poll(
            0,
            Some(ExpectError(USER_ERROR_CODE, &"Poll has already ended")),
        )
        .await;

    // check votes scores

    let poll = interactor.polls(0).await;
    assert!(poll.status == false, "wrong status");
    assert!(
        poll.vote_score.get(0).clone_value() == BigUint::from(1000000000000000000u128),
        "wrong score for option 0"
    );
    assert!(
        poll.vote_score.get(1).clone_value() == BigUint::from(6000000000000000000u128),
        "wrong score for option 1"
    );
    assert!(
        poll.vote_score.get(2).clone_value() == BigUint::from(10000000000000000000u128),
        "wrong score for option 2"
    );
    assert!(
        poll.vote_score.get(3).clone_value() == BigUint::from(0u128),
        "wrong score for option 3"
    );

    // check number of votes per option

    let poll_votes_option_0 = interactor.poll_votes(0, 0).await;
    assert!(
        poll_votes_option_0 == 1,
        "wrong number of votes for option 0"
    );
    let poll_votes_option_1 = interactor.poll_votes(0, 1).await;
    assert!(
        poll_votes_option_1 == 1,
        "wrong number of votes for option 1"
    );
    let poll_votes_option_2 = interactor.poll_votes(0, 2).await;
    assert!(
        poll_votes_option_2 == 4,
        "wrong number of votes for option 2"
    );
    let poll_votes_option_3 = interactor.poll_votes(0, 3).await;
    assert!(
        poll_votes_option_3 == 0,
        "wrong number of votes for option 3"
    );

    // check total number of votes

    let total_votes = interactor.get_total_votes(0).await;
    assert!(total_votes == 6, "wrong total number of votes");
}
