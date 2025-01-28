use ethers::prelude::*;

use crate::Balance;
use std::convert::TryFrom;

pub struct EthereumRepository {
    provider: Provider<Http>,
}

impl EthereumRepository {
    pub fn new() -> Self {
        let provider = Provider::<Http>::try_from("https://rpc.gnosischain.com")
            .expect("Failed to create provider");
        Self { provider }
    }

    pub async fn get_balance(&self, address: &str) -> Result<Balance, ProviderError> {
        let addr: Address = address.parse().expect("Invalid Ethereum address");
        let balance = self.provider.get_balance(addr, None).await?;
        Ok(Balance::from(balance))
    }
}
