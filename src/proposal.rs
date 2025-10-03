use multiversx_sc::imports::*;

use crate::basics::{
    events,
    storage::{self, Proposal},
};

#[multiversx_sc::module]
pub trait ProposalModule:
    storage::StorageModule
    + events::EventsModule
    + multiversx_sc_modules::pause::PauseModule
    + multiversx_sc_modules::only_admin::OnlyAdminModule
{
    #[endpoint(newProposal)]
    fn new_proposal(&self, description: ManagedBuffer) -> usize {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let mut index = 0;
        self.next_available_proposal_index()
            .update(|current_index| {
                index = *current_index;
                *current_index += 1;
            });

        self.new_proposal_event(index, &description);

        self.proposals(index).set(&Proposal {
            initiator: caller,
            description,
            vote_score: BigUint::zero(),
            propose_time: current_timestamp,
        });

        index
    }
}
