#![allow(non_snake_case)]

pub mod config;
pub mod contract_interactions;
pub mod interact;
pub mod interact_cli;
pub mod proxy;
pub mod state;

use clap::Parser;
pub use config::Config;
pub use interact::Interact;
use multiversx_sc::imports::*;
use multiversx_sc_snippets::{env_logger, hex, imports::StaticApi};
pub use state::State;

use crate::interact::HASH_LENGTH;
pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";

pub async fn pulse_sc_cli() {
    env_logger::init();

    let mut interact = Interact::new(Config::load_config()).await;

    let cli = interact_cli::InteractCli::parse();

    match cli.command {
        Some(interact_cli::InteractCliCommand::Deploy) => interact.deploy().await,
        Some(interact_cli::InteractCliCommand::Upgrade) => interact.upgrade().await,
        Some(interact_cli::InteractCliCommand::SetRootHash(args)) => {
            let root_hash = get_managed_byte_array_from_string(&args.root_hash);
            interact.set_root_hash(root_hash).await
        }
        Some(interact_cli::InteractCliCommand::NewPoll(args)) => {
            interact
                .new_poll(
                    &args.question,
                    args.options.iter().map(|s| s.as_str()).collect(),
                    args.duration,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::EndPoll(args)) => {
            interact.end_poll(args.index, None).await
        }
        Some(interact_cli::InteractCliCommand::VotePoll(args)) => {
            let proof: Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> =
                get_proof_from_string(&args.proof);
            let caller = Bech32Address::from_bech32_string(args.caller);
            interact
                .vote_poll(
                    caller,
                    args.index,
                    args.option,
                    args.voting_power,
                    proof,
                    None,
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::Polls(args)) => interact.polls(args.index).await,
        Some(interact_cli::InteractCliCommand::PollVotes(args)) => {
            interact.poll_votes(args.index, args.option).await
        }
        Some(interact_cli::InteractCliCommand::GetTotalVotes(args)) => {
            interact.get_total_votes(args.index).await
        }
        Some(interact_cli::InteractCliCommand::ConfirmVotingPower(args)) => {
            let proof: Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> =
                get_proof_from_string(&args.proof);
            interact
                .confirm_voting_power(args.voting_power, proof)
                .await
        }
        Some(interact_cli::InteractCliCommand::Pause) => interact.pause_endpoint().await,
        Some(interact_cli::InteractCliCommand::Unpause) => interact.unpause_endpoint().await,
        Some(interact_cli::InteractCliCommand::IsPaused) => interact.paused_status().await,

        None => {}
    }
    fn get_proof_from_string(
        string_proof: &str,
    ) -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> {
        let mut proof = Vec::new();

        let proof_bytes = vec![string_proof];

        for bytes in proof_bytes {
            let managed_array_bytes = get_managed_byte_array_from_string(&bytes);
            proof.push(managed_array_bytes);
        }

        proof
    }

    fn get_managed_byte_array_from_string(
        string: &str,
    ) -> ManagedByteArray<StaticApi, { HASH_LENGTH }> {
        let hex_bytes = hex::decode(string).unwrap();
        ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
            &hex_bytes.as_slice().try_into().unwrap(),
        )
    }
}
