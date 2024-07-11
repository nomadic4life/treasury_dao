use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,
    pub token_mint: Pubkey,

    // MINT AUTHORITRY -> may not need, since token will be minted once into vault, and never minted again.
    pub authority: Pubkey,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + BYTE + PUBKEY_BYTES + PUBKEY_BYTES;
}
