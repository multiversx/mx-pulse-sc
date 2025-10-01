use multiversx_sc::imports::*;

use crate::basics::constants::{HASH_LENGTH, ONGOING};
use crate::basics::errors::{ALREADY_VOTED, INVALID_VOTING_POWER, POLL_ENDED};
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

        self.polls(poll_index).update(|poll| {
            require!(poll.status == ONGOING, POLL_ENDED);
            require!(option_index < poll.options.len(), INVALID_VOTING_POWER);

            let mut poll_voters = self.poll_voters(poll_index);
            require!(!poll_voters.contains(&caller), ALREADY_VOTED);

            poll_voters.insert(caller.clone());

            let votes = poll.vote_score.get(option_index).clone() + voting_power;
            let _ = poll.vote_score.set(option_index, votes);
        });
        self.poll_votes(poll_index, option_index)
            .update(|votes| *votes += 1);

        self.vote_cast_event(caller, poll_index, option_index);
    }
}
