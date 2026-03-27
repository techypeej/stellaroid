#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Receipt(Symbol), // Stores receipt hash -> Owner Address
}

#[contract]
pub struct CashtellarContract;

#[contractimpl]
impl CashtellarContract {
    /// Registers a receipt hash to an owner. 
    /// Prevents duplicates and rewards the user with a symbolic 1 XLM (simulated).
    pub fn register_receipt(env: Env, owner: Address, receipt_hash: Symbol) -> Symbol {
        owner.require_auth();

        let key = DataKey::Receipt(receipt_hash.clone());

        // Check if receipt already exists to prevent scam re-use
        if env.storage().persistent().has(&key) {
            panic!("Receipt already validated or duplicate!");
        }

        // Store the ownership: Mapping the unique hash to the sender
        env.storage().persistent().set(&key, &owner);

        // Logic for "Success"
        symbol_short!("SUCCESS")
    }

    /// Checks if a receipt is valid and returns the owner's address
    pub fn verify_receipt(env: Env, receipt_hash: Symbol) -> Address {
        let key = DataKey::Receipt(receipt_hash);
        
        // If the hash isn't in storage, it's either fake or not registered
        env.storage().persistent().get(&key).expect("Receipt not found/Invalid")
    }
}