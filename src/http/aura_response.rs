use serde::Serialize;
use crate::Balance;
use crate::http::balance_response::BalanceResponse;

#[derive(Debug, Serialize)]
pub struct AuraResponse {
    pub balance: BalanceResponse,
    pub earned: BalanceResponse,
}

impl AuraResponse {
    pub fn new(balance: Balance, earned: Balance) -> Self {
        let balance_response = BalanceResponse::from(balance);
        let earned_response = BalanceResponse::from(earned);
        Self { balance: balance_response, earned: earned_response }
    }
}
