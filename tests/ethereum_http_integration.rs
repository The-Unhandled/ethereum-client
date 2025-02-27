#[cfg(test)]
mod tests {
    use ethereum_client::repositories::ethereum_http::EthereumHttpClient;
    use ethers::providers::{Http, Provider};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_get_chainlink_price() {
        // Create a test provider.
        let provider = Arc::new(
            Provider::<Http>::try_from("https://rpc.gnosischain.com")
                .expect("Failed to create test provider")
        );

        // Create our client with the test provider.
        let client = EthereumHttpClient::new_with_provider(provider);

        let price_result = client.get_chainlink_price().await;

        match price_result {
            Ok(price) => {
                println!("Latest price: {:.2} USD", price.to_usd());
                // For the test, we assert that the price is greater than zero.
                assert!(price.to_usd() > 0.0);
            },
            Err(e) => panic!("Failed to get Chainlink price: {:?}", e),
        }
    }
}
