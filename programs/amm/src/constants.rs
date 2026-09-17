use anchor_lang::prelude::*;

#[constant]
pub const POOL_SEED: &[u8] = b"pool";

#[constant]
pub const CONFIG_SEED: &[u8] = b"config";

#[constant]
pub const PRECISION: u64 = 1_000_000_000;

#[constant]
pub const BASE_FEE_BPS: u16 = 30; // 0.3%
