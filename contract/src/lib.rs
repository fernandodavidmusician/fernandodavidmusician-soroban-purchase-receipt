#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, token, Address, Env, String };

// Define enumeration DataKey with all possible values across the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin, // Contract administrator
    Asset, // Native asset address
    Recipient, // Payment recipient address
    Amount, // Payment amount
    Receipt, // Payment receipt
}

// Set of functions which will make the contract more readable and easy to manage

// Get 'asset' address
fn get_asset(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Asset).expect("Not initialized!")
}

// Get 'recipient' address
fn get_recipient(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Recipient).expect("Not Initialized!")
}

// Get payment 'amount'
fn get_amount(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::Amount).expect("Not Initialized!")
}

// Get payment 'receipt'
fn get_receipt(env: &Env) -> String {
    env.storage().instance().get(&DataKey::Receipt).expect("Not Initialized!")
}

// Get contract 'balance'
fn get_balance(env: &Env, asset: &Address) -> i128 {
    let client = token::Client::new(&env, &asset);
    client.balance(&env.current_contract_address())
}

// Transfer 'asset'
fn transfer(env: &Env, recipient: &Address, amount: &i128) {
    let asset = get_asset(env);
    let client = token::Client::new(&env, &asset);
    client.transfer(&env.current_contract_address(), &recipient, &amount)
}

#[contract]
struct Contract;
pub trait ContractTrait {
    
    // Initialize function
    fn init(env: Env, admin: Address, asset: Address, recipient: Address, amount: i128, receipt: String);

    // Update function
    fn update(env: Env, admin: Address, recipient: Address, amount: i128, receipt: String);

    // Pay function
    fn pay(env: Env, payer: Address);

    // Balance function
    fn balance(env: Env, asset: Address) -> i128;
    
    // Receipt function
    fn receipt(env: Env) -> String;

     // Withdraw function
    fn withdraw(env: Env, admin: Address);
}

#[contractimpl]
impl ContractTrait for Contract {

    // Initialize contract
    fn init(env: Env, admin: Address, asset: Address, recipient: Address, amount: i128, receipt: String) {

        // Check 'admin' authorization
        admin.require_auth();

        // Check contract initialization
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized!")
        }

        // Set and save 'admin' address
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Set and save 'asset' address
        env.storage().instance().set(&DataKey::Asset, &asset);

        // Set and save 'recipient' address
        env.storage().instance().set(&DataKey::Recipient, &recipient);

        // Set and save payment 'amount'
        env.storage().instance().set(&DataKey::Amount, &amount);

        // Set and save payment 'receipt'
        env.storage().instance().set(&DataKey::Receipt, &receipt);
    }

    // Update 'amount', 'recipient' and 'receipt'
    fn update(env: Env, admin: Address, recipient: Address, amount: i128, receipt: String) {

        // Check 'admin' authorization
        admin.require_auth();

        // Check contract initialization
        if !env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract not initialized!")
        }

        // Check if 'admin' is the current contract administrator
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if current_admin != admin {
            panic!("Not the current admin!")
        }

        // Set and save new 'recipient' address
        env.storage().instance().set(&DataKey::Recipient, &recipient);

        // Set and save new payment 'amount'
        env.storage().instance().set(&DataKey::Amount, &amount);

        // Set and save new payment 'receipt'
        env.storage().instance().set(&DataKey::Receipt, &receipt);

    }

    // Pay 'asset' to the current contract
    fn pay(env: Env, payer: Address) {

        // Check 'payer' authorization
        payer.require_auth();

        // Transfer 'asset' from 'payer' to the current contract
        let asset = env.storage().instance().get(&DataKey::Asset).unwrap();
        let client = token::Client::new(&env, &asset);
        client.transfer(&payer, &env.current_contract_address(), &get_amount(&env))
    }

    // Get 'asset' balance
    fn balance(env: Env, asset: Address) -> i128 {
        let client = token::Client::new(&env, &asset);
        client.balance(&env.current_contract_address())
    }

    // Get payment 'receipt' from storage
    fn receipt(env: Env) -> String {
        get_receipt(&env)
    }

    // Withdraw 'asset' to the 'recipient'
    fn withdraw(env: Env, admin: Address) {

        // Check 'admin' authorization
        admin.require_auth();

        // Withdraw 'asset' from the current contract to the 'recipient'
        let asset = get_asset(&env);
        let recipient = get_recipient(&env);
        transfer(&env, &recipient, &get_balance(&env, &asset));
    }
}