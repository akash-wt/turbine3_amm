pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9VMgQUvDvYdGNTFmKeZN7y2YY7G99UGGMpz1XtrAPtPA");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, initial_reserves_a: u64, initial_reserves_b: u64, amplification_coefficient: u64) -> Result<()> {
        crate::instructions::initialize_pool::handle_initialize_pool(ctx, initial_reserves_a, initial_reserves_b, amplification_coefficient)
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        crate::instructions::swap::handle_swap(ctx, amount_in, min_amount_out)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        crate::instructions::add_liquidity::handle_add_liquidity(ctx, amount_a, amount_b)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
        crate::instructions::remove_liquidity::handle_remove_liquidity(ctx, lp_amount)
    }
}
