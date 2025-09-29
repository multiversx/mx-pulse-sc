use config::Config;
use multiversx_sc_snippets::{imports::*, sdk::gateway::NetworkStatusRequest};

use crate::{config, State, CHAIN_SIMULATOR_GATEWAY};

pub const HASH_LENGTH: usize = 32;
pub const STATE_FILE: &str = "state.toml";

pub struct Interact {
    pub interactor: Interactor,
    pub owner_address: Address,
    pub contract_code: BytesValue,
    pub state: State,
}

impl Interact {
    pub async fn new(config: Config) -> Self {
        Self::create_interact_with_state(config, State::default()).await
    }

    pub async fn load(config: Config) -> Self {
        Self::create_interact_with_state(config, State::load_state()).await
    }

    async fn create_interact_with_state(config: Config, state: State) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("interactor");

        let owner_address = interactor.register_wallet(test_wallets::alice()).await;

        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/pulse-sc.mxsc.json",
            &InterpreterContext::default(),
        );

        Interact {
            interactor,
            owner_address,
            contract_code,
            state,
        }
    }

    pub async fn generate_blocks_until_epoch(&mut self, epoch: u64) {
        self.interactor
            .generate_blocks_until_epoch(epoch)
            .await
            .unwrap();
    }
    pub async fn generate_blocks(&mut self, num_blocks: u64) {
        self.interactor.generate_blocks(num_blocks).await.unwrap();
    }

    pub async fn generate_blocks_until_next_epoch(&mut self) {
        self.interactor
            .generate_blocks_until_epoch(self.get_next_epoch().await)
            .await
            .unwrap();
    }

    async fn get_next_epoch(&self) -> u64 {
        let blockchain = GatewayHttpProxy::new(CHAIN_SIMULATOR_GATEWAY.to_string());

        let network_config = blockchain
            .http_request(NetworkStatusRequest::default())
            .await
            .unwrap();
        network_config.epoch_number + 1u64
    }
}
