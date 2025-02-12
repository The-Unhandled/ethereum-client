use axum::{serve, Router};
use env_logger;
use ethereum_client::http::ethereum::AppState;
use ethereum_client::http::ethereum_routes;
use ethereum_client::services::ethereum::EthereumService;
use ethereum_client::grpc::aura_service::{AuraServiceServer, AuraServiceImpl};
use std::sync::Arc;
use tokio::net::TcpListener;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Initialize the Ethereum service (shared state)
    let ethereum_service = Arc::new(EthereumService::new());

    // Create application state
    let app_state = AppState {
        ethereum_service: ethereum_service.clone(),
    };

    // Setup Axum HTTP server
    let http_address = "0.0.0.0:3000";
    let http_listener = TcpListener::bind(http_address).await.unwrap();
    let http_app = Router::new()
        .nest("/api/ethereum", ethereum_routes(app_state.clone()));

    println!("HTTP server listening on {}", http_address);

    // Setup Tonic gRPC server
    let grpc_address = "0.0.0.0:50051".parse().unwrap();
    let grpc_service = AuraServiceServer::new(AuraServiceImpl::new(ethereum_service.clone()));
    
    println!("gRPC server listening on {}", grpc_address);

    // Run both servers concurrently
    tokio::select! {
        _ = serve(http_listener, http_app) => {
            println!("HTTP server exited");
        }
        _ = Server::builder()
            .add_service(grpc_service)
            .serve(grpc_address) => {
            println!("gRPC server exited");
        }
    }
}
