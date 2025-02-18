use ethers::prelude::*;
use ethers::providers::Ws;
use ethers::types::Filter;
use std::sync::Arc;
use tokio::task;
use log::{info, error};
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use crate::config::AppConfig;

pub struct EthereumWsClient {
    provider: Arc<Provider<Ws>>,
    kafka_producer: FutureProducer,
    kafka_topic: String
}

impl EthereumWsClient {
    pub async fn new() -> Self {

        let config = AppConfig::load().expect("❌ Failed to load config");

        let provider = Arc::new(
            Provider::<Ws>::connect("wss://rpc.gnosischain.com/wss")
                .await
                .expect("❌ WebSocket connection failed"),
        );

        info!("📡 WS Provider connected");


        let kafka_producer = ClientConfig::new()
            .set("bootstrap.servers", &config.kafka.brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("❌ Failed to create Kafka producer");

        info!("📡 WS Provider connected & Kafka producer ready");

        Self { provider, kafka_producer, kafka_topic: config.kafka.topic.clone() }
    }

    pub fn start_log_listener(&self) {
        let ws_provider = self.provider.clone();
        let producer = self.kafka_producer.clone();
        let topic = self.kafka_topic.clone();
        let filter = Filter::new();

        task::spawn(async move {
            match ws_provider.subscribe_logs(&filter).await {
                Ok(mut stream) => {
                    info!("📡 Listening for Ethereum logs...");
                    while let Some(log) = stream.next().await {
                        let log_str = format!("{:?}", log);
                        info!("🔹 New log: {}", log_str);

                        // ✅ Send log to Kafka using topic from config
                        let record = FutureRecord::to(&topic)
                            .key("eth-log")
                            .payload(&log_str);

                        if let Err(e) = producer.send(record, Timeout::Never).await {
                            error!("❌ Failed to send log to Kafka: {:?}", e);
                        }
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
