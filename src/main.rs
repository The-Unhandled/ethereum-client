use axum::{Router, serve};
use std::sync::Arc;
use tokio::net::TcpListener;
use ethereum_client::routes::ethereum::AppState;
use ethereum_client::routes::ethereum_routes;
use ethereum_client::services::ethereum::EthereumService;

#[tokio::main]
async fn main() {
    // Initialize the Ethereum service
    let ethereum_service = Arc::new(EthereumService::new());

    // Create application state
    let app_state = AppState {
        ethereum_service,
    };

    // Create the Axum app with the routes
    let app = Router::new()
        .nest("/api/ethereum", ethereum_routes(app_state.clone()));

    let address = "0.0.0.0:3000";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening globally on {}", address);
    serve(listener, app).await.unwrap();
}
