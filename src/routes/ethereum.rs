use axum::{routing::get, Router, Json};
use std::sync::Arc;
use axum::extract::{Path, State};
use serde::Serialize;
use crate::Balance;
use crate::services::ethereum::EthereumService;

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

#[derive(Clone)]
pub struct AppState {
    pub ethereum_service: Arc<EthereumService>,
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/balance/{address}", get(get_balance))
        .with_state(state)
}

async fn get_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<BalanceResponse> {
    // Example usage of the EthereumService
    let balance = state
        .ethereum_service
        .get_balance(&address)
        .await
        .unwrap_or_else(|_| Balance::default());
    Json(BalanceResponse::from(balance))
}
