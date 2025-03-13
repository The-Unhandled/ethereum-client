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

abigen!(
    BalancerGauge,
    r#"./src/resources/contracts/balancer_gauge_abi.json"#
);

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

    pub fn start_event_listener(&self) {

        let gauge_address: Address = "0xbcF4969d4dc6Cb86Ce0B8a101d220b558F14739C"
            .parse()
            .expect("Invalid Balancer Gauge address");

        let contract = BalancerGauge::new(gauge_address, self.provider.clone());

        let test_block = BlockNumber::Number(U64::from(39001900));

        let watcher = contract.events().from_block(test_block);

        task::spawn(async move {
            match watcher.subscribe().await {
                Ok(mut stream) => {
                    info!("📡 Listening for Balancer Gauge events...");
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(event) => {
                                match event {
                                    BalancerGaugeEvents::DepositFilter(deposit) => {
                                        info!("🔹 New deposit: {:?}", deposit);
                                        //handle_event("DEPOSIT", &deposit.user, &deposit.value, &meta, &producer, &topic).await;
                                    }
                                    BalancerGaugeEvents::WithdrawFilter(withdraw) => {
                                        info!("🔹 New withdraw: {:?}", withdraw);
                                        //handle_event("WITHDRAW", &withdraw.user, &withdraw.value, &meta, &producer, &topic).await;
                                    }
                                    _ => {} // Ignore other events
                                }
                            }
                            Err(e) => {
                                error!("❌ Error processing event: {:?}", e);
                            }
                        }
                    }
                    error!("❌ Event stream closed.");
                }
                Err(e) => {
                    error!("❌ Failed to subscribe to events: {:?}", e);
                }
            }
        });

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
