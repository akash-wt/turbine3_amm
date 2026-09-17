use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(initial_reserves_a: u64, initial_reserves_b: u64, amplification_coefficient: u64)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = user,
        space = GlobalConfig::LEN,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = user,
        space = Pool::LEN,
        seeds = [POOL_SEED, mint_a.key.as_ref(), mint_b.key.as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub mint_a: AccountInfo<'info>,
    pub mint_b: AccountInfo<'info>,
    pub lp_mint: AccountInfo<'info>,

    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_pool(ctx: Context<InitializePool>, initial_reserves_a: u64, initial_reserves_b: u64, amplification_coefficient: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.treasury = ctx.accounts.treasury.key();
    config.base_fee_bps = BASE_FEE_BPS;
    config.volatility_multiplier = 100; // Default
    config.admin = ctx.accounts.user.key();

    let pool = &mut ctx.accounts.pool;
    pool.reserve_a = initial_reserves_a;
    pool.reserve_b = initial_reserves_b;
    pool.amplification_coefficient = amplification_coefficient;
    pool.total_lp_supply = 0;
    pool.bump = ctx.bumps.pool;

    // Calculate initial invariant D
    // D = (x + y) + A*x*y
    // For simplicity, let's use a simplified version for the first pool
    let a = amplification_coefficient as f64 / PRECISION as f64;
    let d = (initial_reserves_a as f64 + initial_reserves_b as f64) + a * (initial_reserves_a as f64 * initial_reserves_b as f64);
    pool.invariant = d as u128;

    Ok(())
}
