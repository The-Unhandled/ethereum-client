use super::chainlink::{PriceFeedRequest, PriceFeedResponse};
use crate::services::ethereum::EthereumService;
use std::collections::HashMap;
use std::sync::Arc;
use ethers::types::Address;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use crate::grpc::chainlink_service::ChainlinkService;

pub struct ChainlinkServiceImpl {
    ethereum_service: Arc<EthereumService>,
    // A mapping of (chain, pair) -> aggregator address
    aggregator_map: Arc<RwLock<HashMap<(String, String), Address>>>,
}

impl ChainlinkServiceImpl {
    pub fn new(ethereum_service: Arc<EthereumService>) -> Self {
        let mut map = HashMap::new();
        // For now, statically set the aggregator address for GNO/USD on Gnosis.
        let address: Address = "0x22441d81416430A54336aB28765abd31a792Ad37".parse().unwrap();
        map.insert(
            ("Gnosis".to_string(), "GNO/USD".to_string()), address,
        );
        Self {
            ethereum_service,
            aggregator_map: Arc::new(RwLock::new(map)),
        }
    }
}

#[tonic::async_trait]
impl ChainlinkService for ChainlinkServiceImpl {
    async fn get_price_feed(
        &self,
        request: Request<PriceFeedRequest>,
    ) -> Result<Response<PriceFeedResponse>, Status> {
        let req = request.into_inner();
        let key = (req.chain.clone(), req.pair.clone());
        let map = self.aggregator_map.read().await;
        if let Some(aggregator_addr) = map.get(&key) {
            // Call the Ethereum HTTP client to get the Chainlink price.
            // Assume get_chainlink_price accepts aggregator address as a string and returns a Price.
            match self.ethereum_service.get_chainlink_price(*aggregator_addr).await {
                Ok(price) => {
                    let resp = PriceFeedResponse {
                        price: price.to_usd(), // Convert the raw price to a human-friendly number
                    };
                    Ok(Response::new(resp))
                }
                Err(e) => Err(Status::internal(format!(
                    "Failed to fetch price: {:?}",
                    e
                ))),
            }
        } else {
            Err(Status::not_found("Aggregator not found for the specified chain/pair"))
        }
    }
}
