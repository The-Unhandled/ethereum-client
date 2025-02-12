pub mod ethereum;
mod aura_response;
mod errors;
mod balance_response;
// Declare the Ethereum http module

use axum::Router;
use crate::AppState;

// Combine all route handlers into one router
pub fn ethereum_routes(state: AppState) -> Router {
    Router::new().merge(ethereum::routes(state))
}
