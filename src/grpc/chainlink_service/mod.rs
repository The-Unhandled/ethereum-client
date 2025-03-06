// Include the generated Protobuf module for Chainlink
pub mod chainlink {
    tonic::include_proto!("chainlink");
}

// Re-export the gRPC service types for easier use
pub use chainlink::chainlink_service_server::{ChainlinkService, ChainlinkServiceServer};

// Bring the service implementation into scope
mod service;
pub use service::ChainlinkServiceImpl;
