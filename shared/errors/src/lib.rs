#![no_std]
use soroban_sdk::contracterror;

/// Custom error codes for Noether contracts
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NoetherError {
    // General errors (0-99)
    Unauthorized = 1,
    InvalidInput = 2,
    InsufficientBalance = 3,
    InsufficientLiquidity = 4,

    // Position errors (100-199)
    PositionNotFound = 100,
    PositionSizeTooSmall = 101,
    PositionSizeTooLarge = 102,
    InvalidLeverage = 103,
    InsufficientCollateral = 104,
    PositionWouldBeLiquidated = 105,
    InvalidPositionSide = 106,

    // Order errors (200-299)
    OrderNotFound = 200,
    InvalidOrderType = 201,
    OrderPriceTooLow = 202,
    OrderPriceTooHigh = 203,
    OrderExpired = 204,

    // Vault errors (300-399)
    VaultPaused = 300,
    VaultTokenMismatch = 301,
    InvalidFeeRate = 302,
    WithdrawalTooLarge = 303,
    DepositTooSmall = 304,

    // Oracle errors (400-499)
    OraclePriceStale = 400,
    OraclePriceInvalid = 401,
    OracleNotConfigured = 402,
    PriceDeviationTooLarge = 403,

    // Math errors (500-599)
    Overflow = 500,
    Underflow = 501,
    DivisionByZero = 502,
}

