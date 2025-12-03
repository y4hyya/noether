use soroban_sdk::{Address, Env};
use noether_types::VaultConfig;
use noether_errors::NoetherError;
use crate::storage;

pub struct Vault<'a> {
    env: &'a Env,
}

impl<'a> Vault<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }

    pub fn initialize(&self, token: Address) -> Result<(), NoetherError> {
        if storage::has_token(self.env) {
            return Err(NoetherError::InvalidInput);
        }

        storage::set_token(self.env, &token);
        
        // Initialize with zero supply
        storage::set_total_supply(self.env, 0);
        storage::set_total_assets(self.env, 0);
        
        Ok(())
    }

    pub fn deposit(&self, _from: &Address, amount: i128) -> Result<i128, NoetherError> {
        if amount <= 0 {
            return Err(NoetherError::InvalidInput);
        }

        let _token = storage::get_token(self.env)?;
        let total_assets = storage::get_total_assets(self.env)?;
        let total_supply = storage::get_total_supply(self.env)?;

        // Transfer tokens from user to vault
        // Note: In a real implementation, you'd use token client to transfer
        // This is a placeholder structure

        let shares = if total_supply == 0 {
            amount
        } else {
            (amount * total_supply) / total_assets
        };

        storage::set_total_assets(self.env, total_assets + amount);
        storage::set_total_supply(self.env, total_supply + shares);

        Ok(shares)
    }

    pub fn withdraw(&self, _to: &Address, shares: i128) -> Result<(), NoetherError> {
        if shares <= 0 {
            return Err(NoetherError::InvalidInput);
        }

        let total_supply = storage::get_total_supply(self.env)?;
        if shares > total_supply {
            return Err(NoetherError::InsufficientBalance);
        }

        let total_assets = storage::get_total_assets(self.env)?;
        let amount = (shares * total_assets) / total_supply;

        if amount > total_assets {
            return Err(NoetherError::InsufficientBalance);
        }

        storage::set_total_assets(self.env, total_assets - amount);
        storage::set_total_supply(self.env, total_supply - shares);

        // Transfer tokens from vault to user
        // Note: In a real implementation, you'd use token client to transfer

        Ok(())
    }

    pub fn total_assets(&self) -> Result<i128, NoetherError> {
        Ok(storage::get_total_assets(self.env)?)
    }

    pub fn total_supply(&self) -> Result<i128, NoetherError> {
        Ok(storage::get_total_supply(self.env)?)
    }

    pub fn get_config(&self) -> Result<VaultConfig, NoetherError> {
        let token = storage::get_token(self.env)?;
        let total_supply = storage::get_total_supply(self.env)?;
        let total_assets = storage::get_total_assets(self.env)?;
        
        Ok(VaultConfig {
            token,
            total_supply,
            total_assets,
            fee_rate: 30, // 0.3% default fee rate
        })
    }
}

