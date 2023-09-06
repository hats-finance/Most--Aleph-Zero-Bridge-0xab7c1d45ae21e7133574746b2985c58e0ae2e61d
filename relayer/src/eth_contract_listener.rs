use crate::config::Config;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, ProviderError, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;
use thiserror::Error;

// #[derive(Debug, Error)]
// #[error(transparent)]
// #[non_exhaustive]
// pub enum EthListenerError {
//     #[error("provider error")]
//     Provider(#[from] ProviderError),

//     #[error("error when parsing ethereum address")]
//     FromHex(#[from] rustc_hex::FromHexError),
// }

abigen!(
    Flipper,
    r#"[
        event Flip(bool newValue)
    ]"#,
);

pub async fn run(config: Arc<Config>) -> Result<()> {
    let Config {
        eth_node_wss_url,
        eth_contract_address,
        ..
    } = &*config;

    let provider = Provider::<Ws>::connect(eth_node_wss_url).await?;
    let client = Arc::new(provider);
    let address: Address = eth_contract_address.parse()?;

    let contract: Flipper<Provider<Ws>> = Flipper::new(address, client);

    let events = contract.event::<FlipFilter>().from_block(16232696);
    let mut stream = events.stream().await?.take(1);
    Ok(())
}