use crate::http::aura_response::AuraResponse;
use crate::http::balance_response::BalanceResponse;
use crate::http::errors::ApiError;
use crate::services::ethereum::EthereumService;
use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub ethereum_service: Arc<EthereumService>,
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/address/{address}", get(get_balance))
        .route("/balancer/{address}", get(get_balancer_staked_balance))
        .route("/balancer/{address}/rewards", get(get_balancer_rewards))
        .route("/aura/{address}", get(get_aura_balance_and_earned))
        .with_state(state)
}

async fn get_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, ApiError> {
    // Example usage of the EthereumService
    let balance = state
        .ethereum_service
        .get_balance(&address)
        .await
        .map_err(|e| ApiError::InternalError(e))?;
    Ok(Json(BalanceResponse::from(balance)))
}

async fn get_balancer_staked_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, ApiError> {
    // Example usage of the EthereumService
    let balance = state
        .ethereum_service
        .get_balancer_staked_balance(&address)
        .await
        .map_err(|e| ApiError::InternalError(e))?;
    Ok(Json(BalanceResponse::from(balance)))
}

async fn get_aura_balance_and_earned(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AuraResponse>, ApiError> {
    state
        .ethereum_service
        .get_aura_balance_and_earned(&address)
        .await
        .map(|(balance, earned)| Json(AuraResponse::new(balance, earned)))
        .map_err(|e| ApiError::InternalError(e))
}

// Call the EthereumService get_balancer_rewards method
async fn get_balancer_rewards(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, ApiError> {
    let balance_response = state
        .ethereum_service
        .get_balancer_rewards(&address)
        .await
        .map(|rewards| {
            BalanceResponse::from(rewards.first().unwrap())
        })
        .map_err(|e| ApiError::InternalError(e))?;

    Ok(Json(balance_response))

}
