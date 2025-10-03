use multiversx_sc::imports::*;

use crate::basics::{
    constants::HASH_LENGTH,
    errors::INVALID_VOTING_POWER,
    events,
    storage::{self, Proposal},
    views,
};

#[multiversx_sc::module]
pub trait ProposalModule:
    storage::StorageModule
    + events::EventsModule
    + views::ViewsModule
    + multiversx_sc_modules::pause::PauseModule
    + multiversx_sc_modules::only_admin::OnlyAdminModule
{
    #[endpoint(newProposal)]
    fn new_proposal(
        &self,
        description: ManagedBuffer,
        voting_power: BigUint,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) -> usize {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();

        let voting_power_check = self.verify_merkle_proof(&caller, &voting_power, proof);
        require!(voting_power_check, INVALID_VOTING_POWER);

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
