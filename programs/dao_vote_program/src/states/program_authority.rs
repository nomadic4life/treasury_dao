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

    pub max_supply: u64,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + (PUBKEY_BYTES * 8) + UNSIGNED_64;

    pub fn init(
        &mut self,
        bump: u8,
        treasury_vault: Pubkey,
        ballot_vault: Pubkey,
        launch_vault: Pubkey,
        treasury_status: Pubkey,
        treasury_token_mint: Pubkey,
        token_mint: Pubkey,
        token_vault: Pubkey,
        token_status: Pubkey,
    ) {
        self.bump = bump;
        self.treasury_vault = treasury_vault;
        self.treasury_status = treasury_status;

        self.launch_vault = launch_vault;
        self.ballot_vault = ballot_vault;

        self.token_status = token_status;

        self.token_vault = token_vault;
        self.treasury_token_mint = treasury_token_mint;
        self.token_mint = token_mint;
    }
}
