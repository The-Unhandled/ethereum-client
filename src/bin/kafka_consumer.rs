use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;

use futures::StreamExt;
use log::LevelFilter::Info;

#[tokio::main]
async fn main() {

    // Initialize logging
    env_logger::builder().filter_level(Info).init();

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9094") // ✅ Connect to Kafka in Docker
        .set("group.id", "test-group") // Consumer group
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest") // Read from beginning
        .create()
        .expect("❌ Failed to create Kafka consumer");

    consumer.subscribe(&["ethereum-logs"]).expect("❌ Failed to subscribe");

    info!("📡 Listening for messages on 'ethereum-logs'...");

    let mut stream = consumer.stream();
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    info!("🔹 Received: {}", String::from_utf8_lossy(payload));
                }
            }
            Err(e) => error!("❌ Kafka error: {:?}", e),
        }
    }
}
