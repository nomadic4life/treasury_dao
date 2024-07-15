use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

pub const POSITION_PROPOSAL_SEED: &str = "position-proposal";

#[account]
pub struct PositionProposal {
    pub bump: u8,
    pub pool_state: Pubkey,

    pub vote_end_slot: u64,
    pub amount: u64,
    pub vote_yes: u64,
    pub vote_no: u64,
}

impl PositionProposal {
    pub const LEN: usize = DISCRIMINATOR + BYTE + PUBKEY_BYTES + UNSIGNED_64 * 4;
    pub fn init(&mut self, bump: u8, pool_state: Pubkey, amount: u64) -> Result<()> {
        let clock = Clock::get()?;

        self.bump = bump;
        self.pool_state = pool_state;
        self.amount = amount;

        self.vote_end_slot = clock.slot + (216_000 * 7);

        Ok(())
    }

    // need to handle possible interger overflow
    pub fn vote(&mut self, is_yes: bool, amount: u64) {
        if is_yes {
            self.vote_yes += amount;
        } else {
            self.vote_no += amount;
        }
    }

    pub fn is_valid_position(&self) -> Result<bool> {
        let slot = Clock::get()?.slot;
        if !(slot >= self.vote_end_slot) && self.vote_yes > self.vote_no {
            return Ok(false);
        }

        Ok(true)
    }
}
