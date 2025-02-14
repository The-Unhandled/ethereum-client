use ethers::prelude::*;
use ethers::providers::Ws;

use crate::config::AppConfig;
use crate::contracts::aura::AuraContract;
use crate::contracts::balancer::BalancerContract;

use crate::Balance;
use log::{error, info};
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::task;

pub struct EthereumRepository {
    provider: Provider<Http>,
    client: Arc<Provider<Http>>,
    ws_provider: Arc<Provider<Ws>>,
    _balancer_address: Address,
    staking_address: Address,
}

impl EthereumRepository {
    pub async fn new() -> Self {
        let config = AppConfig::load().expect("Failed to load config");

        // ✅ Fail immediately if HTTP provider cannot be created
        let provider = Provider::<Http>::try_from("https://rpc.gnosischain.com")
            .expect("❌ Failed to create provider");

        let client = Arc::new(provider.clone());

        info!("📡 WS Provider connecting...");


        // ✅ Fail immediately if WebSocket connection fails
        let ws_provider = Arc::new(
            Provider::<Ws>::connect("wss://rpc.gnosischain.com/wss")
                .await
                .expect("❌ WebSocket connection failed"),
        );

        info!("📡 WS Provider connected");

        Self {
            provider,
            client,
            ws_provider,
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

    // HTTP

    pub async fn get_balance(&self, address: &str) -> Result<Balance, ProviderError> {
        let addr: Address = address.parse().expect("Invalid Ethereum address");

        info!("Calling Ethereum::get_balance for address: {}", addr);

        let balance = self.provider.get_balance(addr, None).await?;
        Ok(Balance::from(balance))
    }

    pub async fn get_balancer_staked_balance(
        &self,
        user_address: &str,
    ) -> Result<Balance, ProviderError> {
        let account: Address = user_address.parse().expect("Invalid Ethereum address");

        let staking_contract = BalancerContract::new(self.staking_address, self.client.clone());

        info!("Calling Balancer::get_balance for address: {}", account);

        let staked_balance = staking_contract
            .balance_of(account)
            .call()
            .await
            .map_err(|e| ProviderError::CustomError(e.to_string()))?;

        Ok(Balance::from(staked_balance))
    }

    pub async fn get_aura_balance(&self, user_address: &str) -> Result<Balance, ProviderError> {
        let account: Address = user_address.parse().expect("Invalid Ethereum address");

        let aura_address: Address = "0x4bdaaebd01fce060e0075bc577ed0d716c17bd32"
            .parse()
            .unwrap();
        let aura_contract = AuraContract::new(aura_address, self.client.clone());

        info!("Calling Aura::get_balance for address: {}", account);

        let balance = aura_contract
            .balance_of(account)
            .call()
            .await
            .map_err(|e| ProviderError::CustomError(e.to_string()))?;

        Ok(Balance::from(balance))
    }

    pub async fn get_aura_earned(&self, user_address: &str) -> Result<Balance, ProviderError> {
        let account: Address = user_address.parse().expect("Invalid Ethereum address");

        let aura_address: Address = "0x4bdaaebd01fce060e0075bc577ed0d716c17bd32"
            .parse()
            .unwrap();
        let aura_contract = AuraContract::new(aura_address, self.client.clone());

        info!("Calling Aura::earned for address: {}", account);

        let earned = aura_contract
            .earned(account)
            .call()
            .await
            .map_err(|e| ProviderError::CustomError(e.to_string()))?;

        Ok(Balance::from(earned))
    }

    // WS

    /// ✅ Listens for Ethereum logs & processes them
    pub fn start_log_listener(&self) {
        // TODO: why not initialize here?
        let ws_provider = self.ws_provider.clone();

        let aura_address: Address = "0x4bdaaebd01fce060e0075bc577ed0d716c17bd32"
            .parse()
            .expect("Invalid contract address");

//        let filter = Filter::new().address(aura_address);
        let filter = Filter::new();

        task::spawn(async move {
            match ws_provider.subscribe_logs(&filter).await {
                Ok(mut stream) => {
                    info!("📡 Listening for Ethereum logs...");

                    while let Some(log) = stream.next().await {
                        info!("🔹 New log: {:?}", log);
                    }

                    error!("❌ Log stream closed.");
                }
                Err(e) => {
                    error!("❌ Failed to subscribe to logs: {:?}", e);
                }
            }
        });
    }
}
