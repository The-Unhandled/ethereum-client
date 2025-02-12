use super::aura::{AuraRequest, AuraResponse};
use crate::grpc::aura_service::AuraService;
use crate::services::ethereum::EthereumService;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct AuraServiceImpl {
    ethereum_service: Arc<EthereumService>,
}

impl AuraServiceImpl {
    pub fn new(ethereum_service: Arc<EthereumService>) -> Self {
        Self { ethereum_service }
    }
}

#[tonic::async_trait]
impl AuraService for AuraServiceImpl {
    async fn get_aura_balance(
        &self,
        request: Request<AuraRequest>,
    ) -> Result<Response<AuraResponse>, Status> {
        let address = request.into_inner().address;
        println!("Fetching Aura balance & earned rewards for: {}", address);

        let (balance, earned) = self
            .ethereum_service
            .get_aura_balance_and_earned(&address)
            .await
            .map_err(|e| Status::internal(format!("Failed to get Aura data: {}", e)))?;

        let response = AuraResponse {
            balance: balance.to_gwei(),
            earned: earned.to_gwei(),
        };

        Ok(Response::new(response))
    }
}
