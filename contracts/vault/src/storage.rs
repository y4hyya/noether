use soroban_sdk::{symbol_short, Address, Env, Symbol};
use noether_errors::NoetherError;

const TOKEN_KEY: Symbol = symbol_short!("TOKEN");
const SUPPLY_KEY: Symbol = symbol_short!("SUPPLY");
const ASSETS_KEY: Symbol = symbol_short!("ASSETS");

pub fn has_token(env: &Env) -> bool {
    env.storage().instance().has(&TOKEN_KEY)
}

pub fn get_token(env: &Env) -> Result<Address, NoetherError> {
    env.storage()
        .instance()
        .get(&TOKEN_KEY)
        .ok_or(NoetherError::InvalidInput)
}

pub fn set_token(env: &Env, token: &Address) {
    env.storage().instance().set(&TOKEN_KEY, token);
}

pub fn get_total_supply(env: &Env) -> Result<i128, NoetherError> {
    env.storage()
        .instance()
        .get(&SUPPLY_KEY)
        .ok_or(NoetherError::InvalidInput)
}

pub fn set_total_supply(env: &Env, supply: i128) {
    env.storage().instance().set(&SUPPLY_KEY, &supply);
}

pub fn get_total_assets(env: &Env) -> Result<i128, NoetherError> {
    env.storage()
        .instance()
        .get(&ASSETS_KEY)
        .ok_or(NoetherError::InvalidInput)
}

pub fn set_total_assets(env: &Env, assets: i128) {
    env.storage().instance().set(&ASSETS_KEY, &assets);
}

