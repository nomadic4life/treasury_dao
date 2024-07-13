use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProposalConfig {
    pub bump: u8,
    pub index: u64,
}

impl ProposalConfig {
    pub const LEN: usize = DISCRIMINATOR + BYTE + UNSIGNED_64;

    pub fn init(&mut self, bump: u8) {
        self.bump = bump;
        self.index = 0;
    }

    pub fn next(&mut self) {
        self.index += 1;
    }
}
