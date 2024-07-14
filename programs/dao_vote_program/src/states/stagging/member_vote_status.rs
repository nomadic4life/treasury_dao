use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

#[account]
pub struct MemberVoteStatus {
    pub member: Pubkey,
    pub amount: u64,
    pub mulitplier: u8,
    pub secret_vote: Hash,
}

impl MemberVoteStatus {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY_BYTES + UNSIGNED_64 + BYTE + PUBKEY_BYTES;

    pub fn init(&mut self, member: Pubkey, amount: u64, mulitplier: u8, secret_vote: Hash) {
        self.member = member;
        self.amount = amount;
        self.multiplier = multiplier;
        self.secret_vote = secret_vote;
    }

    pub fn vote_weight(&self) -> u128 {
        return self.amount as u128 * self.multiplier as u128;
    }

    pub fn is_valid_vote(&self, secret: Hash, vote: bool) -> bool {
        let mut hasher = Hasher::default();

        hasher.hash(vote.as_ref());
        hasher.hash(self.amount.to_string().as_ref());
        hasher.hash(self.member.as_ref());
        hasher.hash(secret.as_ref());

        let hash = hasher.result();
        return hash == self.secret_vote;
    }

    pub fn tax() {

        // losing side
        // tax rate = amount / 100 * (multiplier / 2 + multiplier + base)

        // winning side
        // tax rate = amount / 100 * (multiplier / 2 + multiplier + base) / 2
    }
}
