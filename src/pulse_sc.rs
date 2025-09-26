#![no_std]
use multiversx_sc::imports::*;
pub mod basics;
pub mod poll;
pub mod vote;

#[multiversx_sc::contract]
pub trait PulseSc:
    poll::PollModule
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
}
