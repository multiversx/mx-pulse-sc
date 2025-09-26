use multiversx_sc::imports::*;

pub type Timestamp = u64;
pub type Status = bool;
pub const ONGOING: bool = true;
pub const ENDED: bool = false;
pub const HASH_LENGTH: usize = 32;
pub type ProposalId = u32;
pub type Hash<M> = ManagedByteArray<M, HASH_LENGTH>;
