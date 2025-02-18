use config::{Config, File};
use serde::Deserialize;

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
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub ethereum: EthereumConfig,
    pub contracts: ContractsConfig,
    pub kafka: KafkaConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let cfg = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        cfg.try_deserialize()
    }
}
