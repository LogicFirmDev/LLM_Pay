# Stellar Soroban Smart Contracts

This folder contains the smart contracts for the LLM Pay project.

## Project Structure

- `src/lib.rs`: The main contract logic.
- `src/test.rs`: Unit tests for the contract.
- `Cargo.toml`: Rust configuration and dependencies.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

## Building the Contract

To build the contract to WebAssembly:

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled contract will be at `target/wasm32-unknown-unknown/release/llm_payment_contract.wasm`.

## Running Tests

To run the unit tests:

```bash
cargo test
```

## Deployment

To deploy to the Stellar Testnet:

```bash
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/llm_payment_contract.wasm \
    --source YOUR_ACCOUNT_NAME \
    --network testnet
```
