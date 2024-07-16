use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

pub const ASSET_INDEXER_SEED: &str = "asset-indexer";

#[account]
pub struct AssetIndexer {
    pub bump: u8,
    pub index: u64,

    // POINTER
    pub current_asset_status: Option<Pubkey>,
}

impl AssetIndexer {
    pub const LEN: usize = DISCRIMINATOR + BYTE + UNSIGNED_64 + (BYTE + PUBKEY_BYTES);

    pub fn init(&mut self, bump: u8, index: u64, asset_status: Pubkey) {
        self.bump = bump;
        self.index = index;
        self.current_asset_status = Some(asset_status);
    }
}
