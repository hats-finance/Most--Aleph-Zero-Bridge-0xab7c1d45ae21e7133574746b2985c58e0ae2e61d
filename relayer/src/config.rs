use std::env;

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub log_level: String,
    pub eth_node_wss_url: String,
    pub eth_contract_address: String,
    pub azero_node_wss_url: String,
    pub azero_sudo_seed: String,

    // TODO: move to DB
    pub eth_from_block: usize,
}

pub trait Load {
    // Static method signature; `Self` refers to the implementor type
    fn load() -> Self;
}

impl Load for Config {
    fn load() -> Config {
        Config {
            log_level: get_env_var("LOG_LEVEL", Some(String::from("info"))),
            eth_node_wss_url: get_env_var("ETH_WSS_URL", Some(String::from("ws://127.0.0.1:8546"))),
            azero_node_wss_url: get_env_var(
                "AZERO_WSS_URL",
                Some(String::from("ws://127.0.0.1:9944")),
            ),
            azero_sudo_seed: get_env_var("AZER_SUDO_SEED", Some(String::from("//Alice"))),
            eth_contract_address: get_env_var("ETH_CONTRACT_ADDRESS", None),
            eth_from_block: get_env_var("ETH_FROM_BLOCK", Some(String::from("0")))
                .parse()
                .expect("Can't parse as int"),
        }
    }
}

fn get_env_var(var: &str, default: Option<String>) -> String {
    match env::var(var) {
        Ok(v) => v,
        Err(_) => match default {
            None => panic!("Missing ENV variable: {var} not defined in environment"),
            Some(d) => d,
        },
    }
}
