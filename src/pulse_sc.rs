#![no_std]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait PulseSc {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
