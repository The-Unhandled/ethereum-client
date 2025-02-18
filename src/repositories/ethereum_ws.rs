use ethers::prelude::*;
use ethers::providers::Ws;
use ethers::types::Filter;
use std::sync::Arc;
use tokio::task;
use log::{info, error};

#[derive(Debug)]
pub struct EthereumWsClient {
    provider: Arc<Provider<Ws>>,
}

impl EthereumWsClient {
    pub async fn new() -> Self {
        let provider = Arc::new(
            Provider::<Ws>::connect("wss://rpc.gnosischain.com/wss")
                .await
                .expect("❌ WebSocket connection failed"),
        );

        info!("📡 WS Provider connected");

        Self { provider }
    }

    pub fn start_log_listener(&self) {
        let ws_provider = self.provider.clone();
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
