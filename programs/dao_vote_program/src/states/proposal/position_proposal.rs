use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

pub const POSITION_PROPOSAL_SEED: &str = "position-proposal";

#[account]
pub struct PositionProposal {
    pub bump: u8,
    pub pool_state: Pubkey,

    // without the data from the pool state
    // there is no way to validate that this
    // input is correct. as of right now,
    // it's good enough for test of concept
    // as we add more features, I will add
    // validations.
    pub input_asset_vault: Pubkey,

    pub deadline: u64,

    // validate the amount from the input asset vault
    pub amount: u64,

    // votes should be u128?
    // or | and 1 token is 1 vote, so the decimals can be truncated
    pub vote_yes_total: u64,
    pub vote_no_total: u64,
}

impl PositionProposal {
    pub const LEN: usize = DISCRIMINATOR + BYTE + PUBKEY_BYTES + PUBKEY_BYTES + UNSIGNED_64 * 4;

    pub fn init(&mut self, bump: u8, pool_state: Pubkey, amount: u64) -> Result<()> {
        let clock = Clock::get()?;

        self.bump = bump;
        self.pool_state = pool_state;
        self.amount = amount;

        self.deadline = clock.slot + (216_000 * 7);

        Ok(())
    }

    // need to handle possible interger overflow
    // also should set this to enum
    pub fn vote(&mut self, is_yes: bool, amount: u64) {
        if is_yes {
            self.vote_yes_total += amount;
        } else {
            self.vote_no_total += amount;
        }
    }

    pub fn is_valid_position(&self) -> Result<bool> {
        let slot = Clock::get()?.slot;
        Ok(slot >= self.deadline && self.vote_yes_total > self.vote_no_total)
    }

    pub fn is_valid_claim(&self) -> Result<bool> {
        let slot = Clock::get()?.slot;
        Ok(slot >= self.deadline)
    }
}
