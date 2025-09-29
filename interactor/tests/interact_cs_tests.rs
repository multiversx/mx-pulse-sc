use multiversx_sc_snippets::imports::*;
use rust_interact::{Config, Interact};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_pulse_sc_cs() {
    let mut interactor = Interact::new(Config::chain_simulator_config()).await;

    interactor.deploy().await;
}
