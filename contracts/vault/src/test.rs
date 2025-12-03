#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NoetherVault);
    let client = NoetherVaultClient::new(&env, &contract_id);

    let token = Address::generate(&env);
    
    client.initialize(&token);
    
    let config = client.get_config();
    assert_eq!(config.token, token);
    assert_eq!(config.total_supply, 0);
    assert_eq!(config.total_assets, 0);
}

#[test]
fn test_deposit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NoetherVault);
    let client = NoetherVaultClient::new(&env, &contract_id);

    let token = Address::generate(&env);
    let user = Address::generate(&env);
    
    client.initialize(&token);
    
    let shares = client.deposit(&user, &1000);
    assert_eq!(shares, 1000);
    
    let config = client.get_config();
    assert_eq!(config.total_assets, 1000);
    assert_eq!(config.total_supply, 1000);
}


