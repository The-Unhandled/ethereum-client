// Re-export all gRPC services
pub mod aura_service;
pub mod chainlink_service;

// Bring the relevant types into scope
pub use aura_service::{AuraServiceImpl, AuraServiceServer};
pub use chainlink_service::{ChainlinkServiceImpl, ChainlinkServiceServer};
