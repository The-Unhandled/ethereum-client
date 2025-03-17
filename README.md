# Ethereum Client

This project is an Ethereum client using `ethers.rs`. 
It provides various functionalities to interact with the Ethereum blockchain and exposes them through HTTP, gRPC, and Kafka.

## Summary

The Ethereum client is built using Rust and the `ethers.rs` library. 
It allows you to interact with the Ethereum blockchain, listen for events, and query balances and prices. 
The client supports both HTTP and WebSocket connections to Ethereum nodes.

## Features

- **HTTP API**: Exposes endpoints to query balances, staked balances, and prices.
- **gRPC API**: Provides gRPC services to query balances and prices.
- **Kafka Integration**: Sends Ethereum logs and events to Kafka topics.

## HTTP Endpoints

The HTTP API is served on port `3000` and provides the following endpoints:

- `GET /api/ethereum/balance/{address}`: Get the balance of the specified address.
- `GET /api/ethereum/balancer_staked_balance/{address}`: Get the staked balance of the specified address in the Balancer pool.
- `GET /api/ethereum/aura_balance_and_earned/{address}`: Get the AURA balance and earned rewards of the specified address.
- `GET /api/ethereum/chainlink_price/{aggregator_address}`: Get the Chainlink price from the specified aggregator address.
- `GET /api/ethereum/balancer_rewards/{address}`: Get the Balancer rewards of the specified address.

## gRPC Services

The gRPC API is served on port `50051` and provides the following services:

- **AuraService**:
    - `GetBalance`: Get the balance of the specified address.
    - `GetEarned`: Get the earned rewards of the specified address.
- **ChainlinkService**:
    - `GetPrice`: Get the Chainlink price from the specified aggregator address.

## Kafka Integration

The client listens for Ethereum logs and specific events (Deposit and Withdraw) from the BalancerGauge contract and sends them to Kafka topics. The Kafka producer is configured with the following settings:

- **Bootstrap Servers**: Configured in the application config.
- **Topic**: Configured in the application config.

## Getting Started

### Prerequisites

- Rust
- Kafka
- Ethereum node (HTTP/WebSocket)

### Running the Application

1. Clone the repository:
   ```sh
   git clone https://github.com/The-Unhandled/ethereum-client.git
   cd ethereum-client
