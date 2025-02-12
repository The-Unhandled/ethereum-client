use serde::Serialize;
use crate::Balance;

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub balance: String,
}

impl From<Balance> for BalanceResponse {
    fn from(balance: Balance) -> Self {
        // Perform the formatting of the balance here
        BalanceResponse {
            balance: format!("{:.2}eth", balance.to_ether()), // Format balance as string
        }
    }
}
