use crate::Balance;
use crate::repositories::ethereum_http::EthereumHttpClient;
use crate::repositories::ethereum_ws::EthereumWsClient;

pub struct EthereumService {
    http_client: EthereumHttpClient,
    ws_client: EthereumWsClient
}

impl EthereumService {
    pub async fn new() -> Self {
        Self {
            http_client: EthereumHttpClient::new(),
            ws_client: EthereumWsClient::new().await,
        }
    }

    pub async fn get_balance(&self, address: &str) -> Result<Balance, String> {
        self.http_client.get_balance(address).await
            .map_err(|e| e.to_string())
    }

    pub async fn get_balancer_staked_balance(&self, address: &str) -> Result<Balance, String> {
        self.http_client.get_balancer_staked_balance(address).await
            .map_err(|e| e.to_string())
    }

    pub async fn get_aura_balance_and_earned(&self, address: &str) -> Result<(Balance, Balance), String> {
        let (balance_result, earned_result) = tokio::join!(
            self.http_client.get_aura_balance(address),
            self.http_client.get_aura_earned(address)
        );

        let balance = balance_result.map_err(|e| e.to_string())?;
        let earned = earned_result.map_err(|e| e.to_string())?;

        Ok((Balance::from(balance), Balance::from(earned)))
    }

    pub fn start_log_listener(&self) {
        self.ws_client.start_log_listener();
    }
}
