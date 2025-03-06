use tonic::transport::Channel;
use std::error::Error;
use ethereum_client::grpc::chainlink_service::chainlink::chainlink_service_client::ChainlinkServiceClient;
use ethereum_client::grpc::chainlink_service::chainlink::PriceFeedRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the gRPC server (adjust the URL/port as needed)
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    let mut client = ChainlinkServiceClient::new(channel);

    // Create the request for the Chainlink price feed for GNO/USD on Gnosis.
    let request = tonic::Request::new(PriceFeedRequest {
        chain: "Gnosis".to_string(),
        pair: "GNO/USD".to_string(),
    });

    // Send the request to the gRPC server.
    let response = client.get_price_feed(request).await?;

    println!("Chainlink price: {:.2} USD", response.into_inner().price);

    Ok(())
}
