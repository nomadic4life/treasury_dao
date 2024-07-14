use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct AssetConfig {
    pub bump: u8,
    pub current_index: u64,

    // TRACKER
    pub last_update_index: u64,
    pub invested_asset_total: u64,
    pub last_slot_range: u64,

    // USDC VALUATION OF ALL ASSETS
    pub valuation_total: u64,
}

impl AssetConfig {
    pub const LEN: usize = DISCRIMINATOR + BYTE + UNSIGNED_64 * 4;

    // right now just returns the current index
    // but in future it will return the next avialable index
    pub fn next_index(&self) -> u64 {
        return self.current_index;
    }

    // in the future will update the index when there is
    // no avialble index.
    pub fn update_index(&mut self) {
        self.current_index += 1;
    }
}
