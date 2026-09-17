use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_vault_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_vault_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, anchor_spl::token::Mint>,
    #[account(mut)]
    pub user_lp_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

pub fn handle_add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    // Calculate LP to mint: (amount_a / reserve_a) * total_lp
    let lp_to_mint = if pool.total_lp_supply == 0 {
        // First depositor gets sqrt(a * b) or similar
        (amount_a as u128 * amount_b as u128).isqrt() as u64
    } else {
        let share_a = (amount_a as u128 * pool.total_lp_supply as u128) / pool.reserve_a as u128;
        let share_b = (amount_b as u128 * pool.total_lp_supply as u128) / pool.reserve_b as u128;
        std::cmp::min(share_a, share_b) as u64
    };

    // Transfer assets to pool
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.user_token_a.to_account_info(),
        to: ctx.accounts.pool_vault_a.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx_a = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_a);
    token::transfer(cpi_ctx_a, amount_a)?;

    let cpi_accounts_b = Transfer {
        from: ctx.accounts.user_token_b.to_account_info(),
        to: ctx.accounts.pool_vault_b.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx_b = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_b);
    token::transfer(cpi_ctx_b, amount_b)?;

    // Mint LP tokens (Implementation of minting would go here)
    // ...

    pool.reserve_a += amount_a;
    pool.reserve_b += amount_b;
    pool.total_lp_supply += lp_to_mint;

    Ok(())
}
