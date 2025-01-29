use ethers::prelude::*;

use crate::config::AppConfig;
use crate::contracts::balancer::BalancerContract;
use crate::Balance;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct EthereumRepository {
    provider: Provider<Http>,
    _balancer_address: Address,
    staking_address: Address,
}

impl EthereumRepository {
    pub fn new() -> Self {
        let config = AppConfig::load().expect("Failed to load config");

        let provider = Provider::<Http>::try_from("https://rpc.gnosischain.com")
            .expect("Failed to create provider");
        Self {
            provider,
            _balancer_address: config
                .contracts
                .balancer_pool
                .parse()
                .expect("Invalid address"),
            staking_address: config
                .contracts
                .staking_contract
                .parse()
                .expect("Invalid address"),
        }
    }

    pub async fn get_balance(&self, address: &str) -> Result<Balance, ProviderError> {
        let addr: Address = address.parse().expect("Invalid Ethereum address");
        let balance = self.provider.get_balance(addr, None).await?;
        Ok(Balance::from(balance))
    }
    
    pub async fn get_balancer_staked_balance(&self, user_address: &str) -> Result<Balance, ProviderError> {
        let account: Address = user_address.parse().expect("Invalid Ethereum address");

        let client = Arc::new(self.provider.clone());
        let staking_contract = BalancerContract::new(self.staking_address, client);

        let staked_balance = staking_contract
            .balance_of(account)
            .call()
            .await
            .map_err(|e| ProviderError::CustomError(e.to_string()))?;

        Ok(Balance::from(staked_balance))
    }
}
