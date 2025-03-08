use serde::Serialize;
use crate::Balance;

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub balance: String,
}

impl From<&Balance> for BalanceResponse {
    fn from(balance: &Balance) -> Self {
        BalanceResponse {
            balance: format!("{:.2}eth", balance.to_ether()),
        }
    }
}

impl From<Balance> for BalanceResponse {
    fn from(balance: Balance) -> Self {
        Self::from(&balance)
    }
}
