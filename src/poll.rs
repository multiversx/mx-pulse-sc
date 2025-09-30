use multiversx_sc::imports::*;

use crate::basics;
use crate::basics::constants::MAX_OPTIONS;
use crate::basics::constants::MIN_OPTIONS;
use crate::basics::constants::ONE_DAY;
use crate::basics::constants::ONE_HOUR;
use crate::basics::errors::DURATION_TOO_LONG;
use crate::basics::errors::DURATION_TOO_SHORT;
use crate::basics::errors::ERROR_INVALID_NUMBER_OPTIONS;
use basics::constants::Timestamp;
use basics::constants::ENDED;
use basics::constants::ONGOING;
use basics::errors::POLL_ENDED;
use basics::errors::POLL_NOT_ENDED;
use basics::events;
use basics::storage;
use basics::storage::Poll;

#[multiversx_sc::module]
pub trait PollModule:
    storage::StorageModule + events::EventsModule + multiversx_sc_modules::pause::PauseModule
{
    #[endpoint(newPoll)]
    fn new_poll(
        &self,
        question: ManagedBuffer,
        options: ManagedVec<ManagedBuffer>,
        duration: Timestamp,
    ) -> usize {
        self.require_not_paused();
        require!(duration >= ONE_HOUR, DURATION_TOO_SHORT);
        require!(duration <= ONE_DAY, DURATION_TOO_LONG);

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let mut index = 0;
        self.next_available_poll_index().update(|current_index| {
            index = *current_index;
            *current_index += 1;
        });
        let num_options = options.len();
        require!(
            num_options >= MIN_OPTIONS && num_options <= MAX_OPTIONS,
            ERROR_INVALID_NUMBER_OPTIONS
        );

        let mut vote_score: ManagedVec<BigUint> = ManagedVec::new();
        for _ in 0..num_options {
            vote_score.push(BigUint::zero());
        }

        self.new_poll_event(index, &question);

        self.polls(index).set(&Poll {
            initiator: caller,
            question,
            options,
            vote_score,
            end_time: current_timestamp + duration,
            status: ONGOING,
        });

        index
    }

    #[endpoint(endPoll)]
    fn end_poll(&self, poll_index: usize) {
        self.require_not_paused();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let mut winner_option_index = 0usize;
        let mut winner_option_vote_count = 0usize;
        let mut winner_option_vote_score = BigUint::zero();

        self.polls(poll_index).update(|poll| {
            require!(poll.status == ONGOING, POLL_ENDED);
            require!(current_timestamp > poll.end_time, POLL_NOT_ENDED);
            poll.status = ENDED;
            let options_range = poll.options.len();

            for option_index in 0..options_range {
                let vote_score = poll.vote_score.get(option_index).clone();
                let votes_count = self.poll_votes(poll_index, option_index).get();
                if vote_score > winner_option_vote_score {
                    winner_option_index = option_index;
                    winner_option_vote_score = vote_score;
                    winner_option_vote_count = votes_count;
                }
            }
        });

        self.poll_voter(poll_index).clear();
        self.poll_ended_event(
            poll_index,
            winner_option_index,
            winner_option_vote_count,
            &winner_option_vote_score,
        );
    }
}
