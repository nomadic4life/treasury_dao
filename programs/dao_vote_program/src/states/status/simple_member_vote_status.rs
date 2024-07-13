use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct MemberVoteStatus {
    pub member: Pubkey,
    pub amount: u64,
    pub multiplier: u8,
}

impl MemberVoteStatus {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY_BYTES + UNSIGNED_64 + BYTE;

    pub fn init(&mut self, member: Pubkey, amount: u64, multiplier: u8) {
        self.member = member;
        self.amount = amount;
        self.multiplier = multiplier;
    }
}
