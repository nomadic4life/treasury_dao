use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]

pub struct AssetVaultStatus {
    pub bump: u8,
    pub token_mint: Pubkey,
    pub vault: Pubkey,

    // VALUE
    pub amount: u64,
    pub valuation: u64,

    // TRACKER
    pub last_slot_update: u64,

    // POINTER
    pub current_index: Option<u64>,
}

impl AssetVaultStatus {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_64
        + (BYTE + UNSIGNED_64);

    pub fn init(&mut self, bump: u8, token_mint: Pubkey, vault: Pubkey, slot: u64, index: u64) {
        self.bump = bump;
        self.token_mint = token_mint;
        self.vault = vault;
        self.amount = 0;
        self.valuation = 0;

        // not sure if this correct
        self.last_slot_update = slot;
        self.current_index = Some(index);
    }
}
