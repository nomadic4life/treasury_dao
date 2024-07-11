use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + BYTE + PUBKEY_BYTES;
}
