use std::usize;

use anchor_lang::prelude::*;

declare_id!("7Hamd5qWYGpTicYuTLmaZrWcZL77Z8JnGboDWbFCNLxF");

#[program]
pub mod kuriakose {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct PoolAccount {
    /// Authority of the pool
    pub pool_authority: Pubkey,

    /// Mint of redeemable tokens
    pub redeemable_mint: Pubkey,

    /// Mint of project tokens
    pub native_mint: Pubkey,

    /// Mint of deposit tokens
    pub deposit_token_mint: Pubkey,

    /// Token account of Pool associated with the project token mint
    pub pool_native: Pubkey,

    /// Token account of Pool associated with deposit token mint
    pub pool_deposit_token: Pubkey,

    /// Total number of native tokens being distributed
    pub total_native_tokens: u64,

    /// Unix timestamp for starting IDO
    pub start_ido_ts: i64,

    /// Unix timestamp for ending IDO
    pub end_ido_ts: i64,

    /// Unix timestamp for withdrawing deposit token from pool
    pub withdraw_deposit_token_rs: i64,

    ///Bump
    pub bump: u8,
}

impl PoolAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH   // Discriminator Length
        + PUBKEY_LENGTH                           // Pool Authority
        + PUBKEY_LENGTH                           // Redeemable Mint
        + PUBKEY_LENGTH                           // deposit token Mint
        + PUBKEY_LENGTH                           // Pool Native Token Account
        + PUBKEY_LENGTH                           // Native Mint
        + PUBKEY_LENGTH                           // Pool deposit token Account
        + DATA_LENGTH_64                          // Total Native Token Amount
        + DATA_LENGTH_64                          // Start IDO TS
        + DATA_LENGTH_64                          // End IDO TS
        + DATA_LENGTH_64                          // Withdraw deposit token TS
        + DATA_LENGTH_8; // Bump
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;
const DATA_LENGTH_64: usize = 8;
const DATA_LENGTH_8: usize = 1;
