use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,
    pub launch_vault_bump: u8,
    pub treasury_vault_bump: u8,
    pub token_mint_bump: u8,
    pub token_vault_bump: u8,

    pub token_mint: Pubkey,
    pub max_supply: u64,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + BYTE + PUBKEY_BYTES + PUBKEY_BYTES;
}
