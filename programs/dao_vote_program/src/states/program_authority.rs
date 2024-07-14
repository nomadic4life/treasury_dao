use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,
    pub ballot_vault: Pubkey,
    pub treasury_vault: Pubkey,
    pub launch_vault: Pubkey,
    pub token_vault: Pubkey,

    pub treasury_token_mint: Pubkey,
    pub token_mint: Pubkey,

    pub treasury_status: Pubkey,
    pub token_status: Pubkey,

    pub token_vault_status_bump: u8,
    pub treasury_status_bump: u8,

    pub max_supply: u64,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + (BYTE * 8) + (PUBKEY_BYTES * 3) + UNSIGNED_64;

    pub fn init(
        &mut self,
        treasury_vault: Pubkey,
        treasury_status: Pubkey,
        treasury_token_mint: Pubkey,
    ) {
        self.treasury_vault = treasury_vault;
        self.treasury_status = treasury_status;
        self.treasury_token_mint = treasury_token_mint;
    }
}
