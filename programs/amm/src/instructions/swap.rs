use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::ErrorCode;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub config: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub user_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_out: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_in: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
    let config = &ctx.accounts.config;

    // Use immutable access for calculations
    let x = ctx.accounts.pool.reserve_a as u128;
    let y = ctx.accounts.pool.reserve_b as u128;
    let gamma = ctx.accounts.pool.amplification_coefficient as u128;
    let d = ctx.accounts.pool.invariant;

    // 1. Calculate Dynamic Fee
    let ratio = x.checked_mul(PRECISION as u128).unwrap() / y;
    let diff = if ratio > PRECISION as u128 {
        ratio - PRECISION as u128
    } else {
        PRECISION as u128 - ratio
    };

    let dynamic_fee_bps = config.base_fee_bps as u128 + (diff * config.volatility_multiplier as u128 / PRECISION as u128) as u16 as u128;
    let fee_amount = (amount_in as u128 * dynamic_fee_bps) / 10000;
    let net_amount_in = amount_in as u128 - fee_amount;

    // 2. Calculate Output using StableSwap Invariant
    let x_prime = x + net_amount_in;
    let numerator = d.checked_sub(x_prime).ok_or(ErrorCode::InvalidAmount)?;
    let denominator = (PRECISION as u128) + (gamma * x_prime / PRECISION as u128);
    let y_prime = numerator / denominator;

    let amount_out = y.checked_sub(y_prime).ok_or(ErrorCode::InsufficientLiquidity)?;

    if amount_out < min_amount_out as u128 {
        return err!(ErrorCode::SlippageExceeded);
    }

    // 3. Transfers
    let cpi_accounts_net = Transfer {
        from: ctx.accounts.user_token_in.to_account_info(),
        to: ctx.accounts.pool.to_account_info(),
        authority: ctx.accounts.user_token_in.to_account_info(),
    };
    let cpi_ctx_net = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_net);
    token::transfer(cpi_ctx_net, net_amount_in as u64)?;

    let cpi_accounts_fee = Transfer {
        from: ctx.accounts.user_token_in.to_account_info(),
        to: ctx.accounts.treasury_token_in.to_account_info(),
        authority: ctx.accounts.user_token_in.to_account_info(),
    };
    let cpi_ctx_fee = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_fee);
    token::transfer(cpi_ctx_fee, fee_amount as u64)?;

    let cpi_accounts_out = Transfer {
        from: ctx.accounts.pool.to_account_info(),
        to: ctx.accounts.user_token_out.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(),
    };
    let cpi_ctx_out = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_out);
    token::transfer(cpi_ctx_out, amount_out as u64)?;

    // 4. Update State
    let pool = &mut ctx.accounts.pool;
    pool.reserve_a = (x + net_amount_in) as u64;
    pool.reserve_b = y_prime as u64;

    Ok(())
}
