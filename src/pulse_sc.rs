#![no_std]
use multiversx_sc::imports::*;
pub mod basics;
pub mod poll;
pub mod proposal;
pub mod vote;

#[multiversx_sc::contract]
pub trait PulseSc:
    poll::PollModule
    + proposal::ProposalModule
    + vote::VoteModule
    + basics::storage::StorageModule
    + basics::events::EventsModule
    + basics::views::ViewsModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[init]
    fn init(&self) {
        self.set_paused(false);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint]
    fn set_root_hash(&self, root_hash: basics::constants::Hash<Self::Api>) {
        self.root_hash().set(&root_hash);
    }
}
