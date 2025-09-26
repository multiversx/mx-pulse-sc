#[multiversx_sc::module]
pub trait EventsModule {
    #[event("poll_ended")]
    fn new_poll_event(&self, #[indexed] poll_index: usize, #[indexed] question: &ManagedBuffer);

    #[event("poll_ended")]
    fn poll_ended_event(
        &self,
        #[indexed] poll_index: usize,
        #[indexed] winning_option_index: usize,
        #[indexed] winning_option_vote_count: usize,
        #[indexed] winning_option_vote_score: &BigUint,
    );
}
