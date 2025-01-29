use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct EthereumConfig {
    pub rpc_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ContractsConfig {
    pub balancer_pool: String,
    pub staking_contract: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub ethereum: EthereumConfig,
    pub contracts: ContractsConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.toml")?;
        let config: AppConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}
