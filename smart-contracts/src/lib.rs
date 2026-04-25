#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env,
};

// Data keys for storage

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Token,
    Balance(Address),
}

#[contract]
pub struct LLMPaymentContract;

#[contractimpl]
impl LLMPaymentContract {
    /// Initialize the contract with an admin and the token to be used for payments.
    pub fn initialize(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// Deposit tokens into the contract to buy credits.
    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let token_addr = env
            .storage()
            .instance()
            .get::<_, Address>(&DataKey::Token)
            .expect("not initialized");

        // Transfer tokens from user to this contract
        let client = soroban_sdk::token::Client::new(&env, &token_addr);
        client.transfer(&user, &env.current_contract_address(), &amount);

        // Update user balance
        let key = DataKey::Balance(user.clone());
        let current_balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        let new_balance = current_balance + amount;
        env.storage().persistent().set(&key, &new_balance);

        // Emit deposit event
        env.events().publish(
            (symbol_short!("deposit"), user),
            amount,
        );
    }

    /// Get the credit balance of a user.
    pub fn get_balance(env: Env, user: Address) -> i128 {
        let key = DataKey::Balance(user);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Get the admin address.
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&DataKey::Admin)
            .expect("not initialized")
    }
}

mod test;
