use multiversx_sc::imports::*;
use multiversx_sc_snippets::{
    imports::{ExpectError, StaticApi},
    *,
};

use crate::{
    interact::HASH_LENGTH,
    proxy::{self, Poll},
    Interact,
};

impl Interact {
    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {new_address_bech32}");
        self.state.set_address(new_address_bech32);
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.owner_address)
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_root_hash(&mut self, root_hash: ManagedByteArray<StaticApi, HASH_LENGTH>) {
        let response = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .set_root_hash(root_hash)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn new_poll(
        &mut self,
        question: &str,
        options: Vec<&str>,
        duration: u64,
        error: Option<ExpectError<'_>>,
    ) {
        let tx = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .new_poll(question, options, duration);

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn end_poll(&mut self, poll_index: u32, error: Option<ExpectError<'_>>) {
        let tx = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .end_poll(poll_index);

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn vote_poll(
        &mut self,
        voter: Bech32Address,
        poll_index: u32,
        option_index: u32,
        voting_power: u128,
        proof: Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>,
        error: Option<ExpectError<'_>>,
    ) {
        let tx = self
            .interactor
            .tx()
            .from(voter)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .vote_poll(poll_index, option_index, voting_power, proof);

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn polls(&mut self, index: u32) -> Poll<StaticApi> {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PulseScProxy)
            .polls(index)
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn poll_votes(&mut self, poll_index: u32, option_index: u32) -> usize {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PulseScProxy)
            .poll_votes(poll_index, option_index)
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn get_total_poll_votes(&mut self, poll_index: u32) -> usize {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PulseScProxy)
            .get_total_poll_votes(poll_index)
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn new_proposal(
        &mut self,
        caller: Bech32Address,
        proposal: &str,
        voting_power: u128,
        proof: Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>,
    ) {
        let tx = self
            .interactor
            .tx()
            .from(&caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .new_proposal(proposal, voting_power, proof)
            .returns(ReturnsResult)
            .run()
            .await;

        println!("Result: {tx:?}");
    }

    pub async fn vote_up_proposal(
        &mut self,
        voter: Bech32Address,
        proposal_index: u32,
        voting_power: u128,
        proof: Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>,
        error: Option<ExpectError<'_>>,
    ) {
        let tx = self
            .interactor
            .tx()
            .from(voter)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .vote_up_proposal(proposal_index, voting_power, proof);

        match error {
            None => {
                tx.returns(ReturnsResultUnmanaged).run().await;
            }
            Some(expect_error) => {
                tx.returns(expect_error).run().await;
            }
        }
    }

    pub async fn get_proposal_vote_ups(&mut self, proposal_index: u32) -> usize {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PulseScProxy)
            .get_proposal_vote_ups(proposal_index)
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn confirm_voting_power(
        &mut self,
        voting_power: u128,
        proof: Vec<ManagedByteArray<StaticApi, { HASH_LENGTH }>>,
    ) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PulseScProxy)
            .confirm_voting_power(voting_power, proof)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn pause_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .pause_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unpause_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::PulseScProxy)
            .unpause_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn paused_status(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PulseScProxy)
            .paused_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
