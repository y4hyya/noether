#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};
use noether_types::VaultConfig;
use noether_errors::NoetherError;

mod storage;
mod vault;

use vault::Vault;

#[contract]
pub struct NoetherVault;

#[contractimpl]
impl NoetherVault {
    /// Initialize the vault with a token address
    pub fn initialize(env: Env, token: Address) -> Result<(), NoetherError> {
        let vault = Vault::new(&env);
        vault.initialize(token)?;
        Ok(())
    }

    /// Deposit tokens into the vault
    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<i128, NoetherError> {
        let vault = Vault::new(&env);
        vault.deposit(&from, amount)
    }

    /// Withdraw tokens from the vault
    pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), NoetherError> {
        let vault = Vault::new(&env);
        vault.withdraw(&to, amount)
    }

    /// Get the total assets in the vault
    pub fn total_assets(env: Env) -> Result<i128, NoetherError> {
        let vault = Vault::new(&env);
        Ok(vault.total_assets()?)
    }

    /// Get the total supply of vault tokens
    pub fn total_supply(env: Env) -> Result<i128, NoetherError> {
        let vault = Vault::new(&env);
        Ok(vault.total_supply()?)
    }

    /// Get vault configuration
    pub fn get_config(env: Env) -> Result<VaultConfig, NoetherError> {
        let vault = Vault::new(&env);
        vault.get_config()
    }
}

#[cfg(test)]
mod test;

