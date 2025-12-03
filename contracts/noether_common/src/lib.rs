#![no_std]
use soroban_sdk::{contracttype, Address};

/// Asset enum representing supported assets
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum Asset {
    Stellar,
    USDC,
}

/// Direction enum for position direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum Direction {
    Long,
    Short,
}

/// Position struct representing a trading position
#[derive(Clone, Debug)]
#[contracttype]
pub struct Position {
    pub owner: Address,
    pub collateral: i128,
    pub size: i128,
    pub entry_price: i128,
    pub liquidation_price: i128,
}

/// Error enum for common contract errors
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum Error {
    SlippageExceeded,
    OracleStale,
    PriceDivergence,
    OverLeveraged,
}

