use crate::repositories::ethereum::EthereumRepository;
use crate::Balance;

pub struct EthereumService {
    repo: EthereumRepository,
}

impl EthereumService {
    pub fn new() -> Self {
        Self {
            repo: EthereumRepository::new(),
        }
    }

    pub async fn get_balance(&self, address: &str) -> Result<Balance, String> {
        self.repo.get_balance(address).await
            .map_err(|e| e.to_string())
    }

    pub async fn get_balancer_staked_balance(&self, address: &str) -> Result<Balance, String> {
        self.repo.get_balancer_staked_balance(address).await
            .map_err(|e| e.to_string())
    }
    
    pub async fn get_aura_balance_and_earned(&self, address: &str) -> Result<(Balance, Balance), String> {
        let (balance_result, earned_result) = tokio::join!(
            self.repo.get_aura_balance(address),
            self.repo.get_aura_earned(address)
        );

        let balance = balance_result.map_err(|e| e.to_string())?;
        let earned = earned_result.map_err(|e| e.to_string())?;

        Ok((Balance::from(balance), Balance::from(earned)))
    }
    
}
