#![no_std]
use soroban_sdk::{contracttype, Address};

/// Position side (Long or Short)
#[derive(Clone)]
#[contracttype]
pub enum PositionSide {
    Long,
    Short,
}

/// Position data structure
#[derive(Clone)]
#[contracttype]
pub struct Position {
    pub user: Address,
    pub collateral_token: Address,
    pub index_token: Address,
    pub side: PositionSide,
    pub size: i128,        // Position size in smallest unit
    pub collateral: i128,  // Collateral amount
    pub entry_price: i128, // Entry price in smallest unit
    pub leverage: i128,    // Leverage multiplier (e.g., 10 = 10x)
}

/// Order type
#[derive(Clone)]
#[contracttype]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
}

/// Order data structure
#[derive(Clone)]
#[contracttype]
pub struct Order {
    pub user: Address,
    pub collateral_token: Address,
    pub index_token: Address,
    pub side: PositionSide,
    pub order_type: OrderType,
    pub size: i128,
    pub price: Option<i128>, // None for market orders
    pub collateral: i128,
    pub leverage: i128,
}

/// Vault configuration
#[derive(Clone)]
#[contracttype]
pub struct VaultConfig {
    pub token: Address,
    pub total_supply: i128,
    pub total_assets: i128,
    pub fee_rate: i128, // Fee rate in basis points (e.g., 30 = 0.3%)
}

/// Oracle price data
#[derive(Clone)]
#[contracttype]
pub struct PriceData {
    pub price: i128,
    pub timestamp: u64,
    pub decimals: u32,
}

