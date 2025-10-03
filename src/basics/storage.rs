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

#[type_abi]
#[derive(Debug, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Proposal<M: ManagedTypeApi> {
    pub initiator: ManagedAddress<M>,
    pub description: ManagedBuffer<M>,
    pub vote_score: BigUint<M>,
    pub propose_time: Timestamp,
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
    fn poll_voters(&self, poll_index: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNextAvailablePollIndex)]
    #[storage_mapper("nextAvailablePollIndex")]
    fn next_available_poll_index(&self) -> SingleValueMapper<usize>;

    #[view(getRootHash)]
    #[storage_mapper("rootHash")]
    fn root_hash(&self) -> SingleValueMapper<Hash<Self::Api>>;

    #[view(getProposal)]
    #[storage_mapper("proposals")]
    fn proposals(&self, index: usize) -> SingleValueMapper<Proposal<Self::Api>>;

    #[storage_mapper("proposalUpVoters")]
    fn proposal_up_voters(&self, proposal_index: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNextAvailableProposalIndex)]
    #[storage_mapper("nextAvailableProposalIndex")]
    fn next_available_proposal_index(&self) -> SingleValueMapper<usize>;
}
