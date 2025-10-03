use multiversx_sc::imports::*;

use crate::basics::constants::{HASH_LENGTH, ONGOING};
use crate::basics::errors::{
    ALREADY_VOTED, INVALID_OPTION_INDEX, INVALID_POLL_INDEX, INVALID_VOTING_POWER, POLL_ENDED,
};
use crate::{basics::events, basics::storage, basics::views};

#[multiversx_sc::module]
pub trait VoteModule:
    storage::StorageModule
    + multiversx_sc_modules::pause::PauseModule
    + views::ViewsModule
    + events::EventsModule
{
    #[endpoint]
    fn vote_poll(
        &self,
        poll_index: usize,
        option_index: usize,
        voting_power: BigUint,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();
        let voting_power_check = self.verify_merkle_proof(&caller, &voting_power, proof);
        require!(voting_power_check, INVALID_VOTING_POWER);
        require!(!self.polls(poll_index).is_empty(), INVALID_POLL_INDEX);

        self.polls(poll_index).update(|poll| {
            require!(poll.status == ONGOING, POLL_ENDED);
            require!(option_index < poll.options.len(), INVALID_OPTION_INDEX);

            let vote_success = self.poll_voters(poll_index).insert(caller.clone());
            require!(vote_success, ALREADY_VOTED);

            let votes = poll.vote_score.get(option_index).clone() + voting_power;
            let _ = poll.vote_score.set(option_index, votes);
        });
        self.poll_votes(poll_index, option_index)
            .update(|votes| *votes += 1);

        self.poll_vote_cast_event(caller, poll_index, option_index);
    }

    #[endpoint]
    fn vote_up_proposal(
        &self,
        proposal_index: usize,
        voting_power: BigUint,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();
        let voting_power_check = self.verify_merkle_proof(&caller, &voting_power, proof);
        require!(voting_power_check, INVALID_VOTING_POWER);
        require!(
            !self.proposals(proposal_index).is_empty(),
            INVALID_POLL_INDEX
        );

        let vote_success = self
            .proposal_up_voters(proposal_index)
            .insert(caller.clone());
        require!(vote_success, ALREADY_VOTED);

        self.proposals(proposal_index).update(|proposal| {
            proposal.vote_score += &voting_power;
        });

        self.proposal_vote_cast_event(caller, proposal_index);
    }
}
