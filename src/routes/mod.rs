pub mod ethereum; // Declare the Ethereum routes module

use axum::Router;
use crate::AppState;

// Combine all route handlers into one router
pub fn ethereum_routes(state: AppState) -> Router {
    Router::new().merge(ethereum::routes(state))
}
