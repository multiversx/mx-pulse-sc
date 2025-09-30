use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

use crate::basics::constants::Hash;
use crate::basics::constants::Status;
use crate::basics::constants::Timestamp;

pub const NO_PROPOSAL: &[u8] = b"Proposal does not exist";

#[type_abi]
#[derive(Debug, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Poll<M: ManagedTypeApi> {
    pub initiator: ManagedAddress<M>,
    pub question: ManagedBuffer<M>,
    pub options: ManagedVec<M, ManagedBuffer<M>>,
    pub vote_score: ManagedVec<M, BigUint<M>>,
    pub end_time: Timestamp,
    pub status: Status,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getPoll)]
    #[storage_mapper("polls")]
    fn polls(&self, index: usize) -> SingleValueMapper<Poll<Self::Api>>;

    #[view(getPollVotes)]
    #[storage_mapper("pollVotes")]
    fn poll_votes(&self, poll_index: usize, option_index: usize) -> SingleValueMapper<usize>;

    #[storage_mapper("pollVoters")]
    fn poll_voter(&self, poll_index: usize) -> SingleValueMapper<ManagedVec<ManagedAddress>>;

    #[view(getNextAvailablePollIndex)]
    #[storage_mapper("nextAvailablePollIndex")]
    fn next_available_poll_index(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("rootHash")]
    fn root_hash(&self) -> SingleValueMapper<Hash<Self::Api>>;
}
