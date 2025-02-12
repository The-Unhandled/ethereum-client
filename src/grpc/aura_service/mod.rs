// Include the generated Protobuf module for Aura
pub mod aura {
    tonic::include_proto!("aura");
}

// Re-export the gRPC service types
pub use aura::aura_service_server::{AuraService, AuraServiceServer};

// Bring the actual service implementation into scope
mod service;
pub use service::AuraServiceImpl;
