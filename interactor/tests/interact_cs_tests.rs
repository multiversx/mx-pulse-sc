mod merkle_proof_setup;
use merkle_proof_setup::MerkleProofs;
use std::vec;

use multiversx_sc_snippets::{hex, imports::*};
use rust_interact::{Config, Interact};

use crate::merkle_proof_setup::MerkleProofSetup;

const ONE_HOUR: u64 = 60 * 60;
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
        )
        .await;

    let allice = test_wallets::alice().to_address();
    interactor
        .vote_poll(
            Bech32Address::from(&allice),
            0,
            1,
            1000000000000000000,
            merkle_proofs.pairs[&allice].clone(),
            None,
        )
        .await;
}
