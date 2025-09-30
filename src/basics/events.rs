#[multiversx_sc::module]
pub trait EventsModule {
    #[event("new_poll")]
    fn new_poll_event(&self, #[indexed] poll_index: usize, #[indexed] question: &ManagedBuffer);

    #[event("vote_cast")]
    fn vote_cast_event(&self, #[indexed] voter: ManagedAddress, #[indexed] poll_index: usize);

    #[event("poll_ended")]
    fn poll_ended_event(
        &self,
        #[indexed] poll_index: usize,
        #[indexed] winning_option_index: usize,
        #[indexed] winning_option_vote_count: usize,
        #[indexed] winning_option_vote_score: &BigUint,
    );
}
