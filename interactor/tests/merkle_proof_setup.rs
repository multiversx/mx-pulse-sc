use std::{collections::HashMap, vec};

use multiversx_sc_snippets::{hex, imports::*};

const HASH_LENGTH: usize = 32;

pub trait MerkleProofSetup {
    fn new() -> Self;
    fn vec_to_proof(&self, vec: Vec<&str>) -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>;

    fn get_wallets_and_proof_pairs(
        &self,
    ) -> HashMap<Address, Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>>;
}

pub struct MerkleProofs {
    pub pairs: HashMap<Address, Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>>,
}

impl MerkleProofSetup for MerkleProofs {
    fn new() -> Self {
        let pairs = Self::get_wallets_and_proof_pairs(&Self {
            pairs: HashMap::new(),
        });
        MerkleProofs { pairs }
    }
    fn vec_to_proof(&self, vec: Vec<&str>) -> Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>> {
        let mut proof = Vec::new();

        for bytes in vec {
            let hex_bytes = hex::decode(bytes).unwrap();
            let managed_array_bytes =
                ManagedByteArray::<StaticApi, { HASH_LENGTH }>::new_from_bytes(
                    &hex_bytes.as_slice().try_into().unwrap(),
                );
            proof.push(managed_array_bytes);
        }

        proof
    }

    fn get_wallets_and_proof_pairs(
        &self,
    ) -> HashMap<Address, Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>> {
        let mut hm = HashMap::new();
        hm.insert(
            test_wallets::alice().to_address(),
            self.vec_to_proof(vec![
                "330f8db028b7b5a9435a0ddfd012bd29996fa9e38bfbf65ea32872c3468a06cb",
                "972e54453b055faafc5d24d7486e7377cfce3d82a94f2d1dd6143ae7f9ddd06d",
                "9b3c15e802052c3b7687dc35da074dffc5675501c8f924478cb98c97b92a0db2",
            ]),
        );
        hm.insert(
            test_wallets::bob().to_address(),
            self.vec_to_proof(vec![
                "8a0224c405c0d3187d06af8d6457667891f48c071ec3974089ab9d547ef62d8c",
                "972e54453b055faafc5d24d7486e7377cfce3d82a94f2d1dd6143ae7f9ddd06d",
                "9b3c15e802052c3b7687dc35da074dffc5675501c8f924478cb98c97b92a0db2",
            ]),
        );
        hm.insert(
            test_wallets::carol().to_address(),
            self.vec_to_proof(vec![
                "9f902af412da3e97363adc54ed95c6d4fc0f42927cc0a334de441e5d6482f00e",
                "0da0c9e8c35137369dedf7ac044fa6f428773d7f278e5e28fb639737de7b6b4c",
                "9b3c15e802052c3b7687dc35da074dffc5675501c8f924478cb98c97b92a0db2",
            ]),
        );
        hm.insert(
            test_wallets::dan().to_address(),
            self.vec_to_proof(vec![
                "7128b41b5286abd5297a295abda74a76da8bc1131e01ed2dcbeb03fc62b0cb56",
                "0da0c9e8c35137369dedf7ac044fa6f428773d7f278e5e28fb639737de7b6b4c",
                "9b3c15e802052c3b7687dc35da074dffc5675501c8f924478cb98c97b92a0db2",
            ]),
        );
        hm.insert(
            test_wallets::eve().to_address(),
            self.vec_to_proof(vec![
                "40f134d758ef498adc00d1213b3144a82cfd7c4773c6eefcdd3cc0031363d491",
                "dda960aa94220ece182849554a342cc64df29ba52d5a0eca93445af6b760df8b",
                "0376c3f342a3070bf4ba3e374a8ee29cc04d70b7b05ea3f936f9a2ca855ea1aa",
            ]),
        );
        hm.insert(
            test_wallets::frank().to_address(),
            self.vec_to_proof(vec![
                "a275c9d6cbc5d4fdc5abd1233aa1c02c562083b8d7279357e77f55c9e469b725",
                "dda960aa94220ece182849554a342cc64df29ba52d5a0eca93445af6b760df8b",
                "0376c3f342a3070bf4ba3e374a8ee29cc04d70b7b05ea3f936f9a2ca855ea1aa",
            ]),
        );
        hm.insert(
            test_wallets::grace().to_address(),
            self.vec_to_proof(vec![
                "ef2c49ff8c5ecdc31ea59284aa87083663d9f546785ce1c989cd6a678f4ae724",
                "0376c3f342a3070bf4ba3e374a8ee29cc04d70b7b05ea3f936f9a2ca855ea1aa",
            ]),
        );
        hm
    }
}
