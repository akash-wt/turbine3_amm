use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
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
    pub user_lp_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

pub fn handle_remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
    // Use immutable access for calculations
    let amount_a = (lp_amount as u128 * ctx.accounts.pool.reserve_a as u128 / ctx.accounts.pool.total_lp_supply as u128) as u64;
    let amount_b = (lp_amount as u128 * ctx.accounts.pool.reserve_b as u128 / ctx.accounts.pool.total_lp_supply as u128) as u64;

    // Burn LP tokens (Implementation would go here)
    // ...

    // Transfer assets to user
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.pool_vault_a.to_account_info(),
        to: ctx.accounts.user_token_a.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(), // Pool is authority
    };
    let cpi_ctx_a = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_a);
    token::transfer(cpi_ctx_a, amount_a)?;

    let cpi_accounts_b = Transfer {
        from: ctx.accounts.pool_vault_b.to_account_info(),
        to: ctx.accounts.user_token_b.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(), // Pool is authority
    };
    let cpi_ctx_b = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts_b);
    token::transfer(cpi_ctx_b, amount_b)?;

    // Update State
    let pool = &mut ctx.accounts.pool;
    pool.reserve_a -= amount_a;
    pool.reserve_b -= amount_b;
    pool.total_lp_supply -= lp_amount;

    Ok(())
}
