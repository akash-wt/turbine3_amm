use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient liquidity in the pool")]
    InsufficientLiquidity,
    #[msg("The swap would exceed the specified slippage tolerance")]
    SlippageExceeded,
    #[msg("The provided amount is invalid")]
    InvalidAmount,
    #[msg("Unauthorized access to the pool configuration")]
    Unauthorized,
}
