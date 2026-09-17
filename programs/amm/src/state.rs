use anchor_lang::prelude::*;

#[account]
pub struct GlobalConfig {
    pub treasury: Pubkey,
    pub base_fee_bps: u16,
    pub volatility_multiplier: u64,
    pub admin: Pubkey,
}

impl GlobalConfig {
    pub const LEN: usize = 8 + 32 + 2 + 8 + 32;
}

#[account]
pub struct Pool {
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub amplification_coefficient: u64, // Scaled by PRECISION
    pub invariant: u128,
    pub total_lp_supply: u64,
    pub bump: u8,
}

impl Pool {
    pub const LEN: usize = 8 + 8 + 8 + 32 + 32 + 8 + 16 + 8 + 1;
}
