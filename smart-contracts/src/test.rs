#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{Address, Env, IntoVal};

#[test]
fn test_payment_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token_admin = Address::generate(&env);

    // Register the payment contract
    let contract_id = env.register_contract(None, LLMPaymentContract);
    let client = LLMPaymentContractClient::new(&env, &contract_id);

    // Register a token contract to use for payments
    // We use the built-in token contract for testing
    let token_id = env.register_stellar_asset_contract(token_admin);
    let token = soroban_sdk::token::Client::new(&env, &token_id);
    let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);

    // Mint some tokens to the user
    token_admin_client.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);

    // Initialize the payment contract
    client.initialize(&admin, &token_id);
    assert_eq!(client.get_admin(), admin);

    // User deposits 100 tokens
    client.deposit(&user, &100);

    // Verify balances
    assert_eq!(token.balance(&user), 900);
    assert_eq!(token.balance(&contract_id), 100);
    assert_eq!(client.get_balance(&user), 100);

    // Verify events
    let events = env.events().all();
    let last_event = events.last().unwrap();
    assert_eq!(
        last_event,
        (
            contract_id.clone(),
            (symbol_short!("deposit"), user.clone()).into_val(&env),
            100i128.into_val(&env)
        )
    );
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_already_initialized() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let contract_id = env.register_contract(None, LLMPaymentContract);
    let client = LLMPaymentContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token);
    client.initialize(&admin, &token);
}
