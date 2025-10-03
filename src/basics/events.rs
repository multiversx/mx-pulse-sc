#[multiversx_sc::module]
pub trait EventsModule {
    #[event("new_poll")]
    fn new_poll_event(&self, #[indexed] poll_index: usize, #[indexed] question: &ManagedBuffer);

    #[event("poll_vote_cast")]
    fn poll_vote_cast_event(
        &self,
        #[indexed] voter: &ManagedAddress,
        #[indexed] poll_index: usize,
        #[indexed] option_index: usize,
        #[indexed] voting_power: &BigUint,
    );

    #[event("poll_ended")]
    fn poll_ended_event(
        &self,
        #[indexed] poll_index: usize,
        #[indexed] winning_option_index: usize,
        #[indexed] winning_option_vote_count: usize,
        #[indexed] winning_option_vote_score: &BigUint,
    );

    #[event("new_proposal")]
    fn new_proposal_event(
        &self,
        #[indexed] proposal_index: usize,
        #[indexed] description: &ManagedBuffer,
    );

    #[event("proposal_vote_up_cast")]
    fn proposal_vote_cast_event(
        &self,
        #[indexed] voter: &ManagedAddress,
        #[indexed] proposal_index: usize,
        #[indexed] voting_power: &BigUint,
    );
}
