use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct PositionProposal {
    pub bump: u8,
    pub pool_state: Pubkey,

    pub amount: u64,
    pub vote_yes: u64,
    pub vote_no: u64,
}

impl PositionProposal {
    pub const LEN: usize = DISCRIMINATOR + BYTE + PUBKEY_BYTES + UNSIGNED_64 * 3;
    pub fn init(&mut self, bump: u8, pool_state: Pubkey, amount: u64) {
        self.bump = bump;
        self.pool_state = pool_state;
        self.amount = amount
    }
}
