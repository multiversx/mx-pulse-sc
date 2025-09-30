use multiversx_sc::imports::*;

use crate::{basics::constants::HASH_LENGTH, basics::storage};

#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {
    #[view(getTotalVotes)]
    fn get_total_votes(&self, poll_index: usize) -> usize {
        let poll = self.polls(poll_index).get();
        let mut total_votes = 0;
        for option_index in 0..poll.options.len() {
            total_votes += self.poll_votes(poll_index, option_index).get();
        }
        total_votes
    }

    #[view(confirmVotingPower)]
    fn confirm_voting_power(
        &self,
        voting_power: BigUint<Self::Api>,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) -> bool {
        let caller = self.blockchain().get_caller();
        self.verify_merkle_proof(&caller, &voting_power, proof)
    }

    fn verify_merkle_proof(
        &self,
        caller: &ManagedAddress,
        power: &BigUint<Self::Api>,
        proof: ManagedVec<ManagedByteArray<HASH_LENGTH>>,
    ) -> bool {
        let mut leaf_bytes = caller.as_managed_buffer().clone();
        let root_hash = self.root_hash().get();

        let p = power.to_bytes_be_buffer();
        leaf_bytes.append(&p);

        let mut hash = self.crypto().sha256(&leaf_bytes);
        for proof_item in proof {
            if BigUint::from(hash.as_managed_buffer())
                < BigUint::from(proof_item.as_managed_buffer())
            {
                let mut tst = hash.as_managed_buffer().clone();
                tst.append(proof_item.as_managed_buffer());

                hash = self.crypto().sha256(tst);
            } else {
                let mut tst = proof_item.as_managed_buffer().clone();
                tst.append(hash.as_managed_buffer());

                hash = self.crypto().sha256(tst);
            }
        }

        hash == root_hash
    }
}
