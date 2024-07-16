use crate::constants::*;
use crate::states::*;
use anchor_lang::prelude::*;

pub const MEMBER_EARN_TOKEN_STATUS_SEED: &str = "member-earn-token-status";

#[account]
pub struct MemberTokenStatus {
    pub authority: Pubkey,
    pub last_round: Option<u64>,
    pub balance: u64,
}

impl MemberTokenStatus {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY_BYTES + (BYTE + UNSIGNED_64) + UNSIGNED_64;
    const MAX: u64 = 20;
    const PERCENT_SHIFT: u64 = 100_00;
    const MEMBER_RATE: u64 = 80;
    const NON_MEMBER_RATE: u64 = 60;

    pub fn init(&mut self, member: Pubkey) {
        self.authority = member;
        self.balance = 0;
        self.last_round = None;
    }

    pub fn update(&mut self, vault_status: &TokenStatus, is_treasury_member: bool) {
        let target = u64::from_be_bytes(vault_status.current_round);

        // need update in all member status updates
        if !(target > self.last_round.unwrap()) {
            // emit log -> showing now update took place
            return;
        }

        let advance = if target - self.last_round.unwrap() <= MemberTokenStatus::MAX {
            1
        } else {
            (target - self.last_round.unwrap()) / MemberTokenStatus::MAX
        };

        for _ in 0..MemberTokenStatus::MAX {
            let round = self.last_round.unwrap();
            if round >= target {
                self.last_round = Some(target);
                break;
            }

            self.balance = self.value(round, vault_status, is_treasury_member);
            self.last_round = Some(round + advance);
        }
    }

    pub fn withdraw(&mut self, amount: u64, vault_status: &TokenStatus, is_treasury_member: bool) {
        self.update(vault_status, is_treasury_member);

        if amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn deposit(&mut self, amount: u64, vault_status: &TokenStatus, is_treasury_member: bool) {
        if self.last_round.is_none() {
            let current_round = u64::from_be_bytes(vault_status.current_round);
            self.balance = amount;
            self.last_round = Some(current_round + 1);
            return;
        }

        self.update(vault_status, is_treasury_member);

        self.balance += amount;
    }

    // we lose percision because using u64,
    // because of possible overflow issue
    // can get better percision using 128
    // and get ratio by n * rate / 100.00%
    pub fn share(&self, starting_balance: u64, is_treasury_member: bool) -> u64 {
        let mut share = self.balance * MemberTokenStatus::PERCENT_SHIFT / starting_balance;
        if !is_treasury_member {
            share = share / MemberTokenStatus::PERCENT_SHIFT * MemberTokenStatus::NON_MEMBER_RATE;
        } else {
            share = share / MemberTokenStatus::PERCENT_SHIFT * MemberTokenStatus::MEMBER_RATE;
        }

        return share;
    }

    pub fn value(&self, round: u64, vault_status: &TokenStatus, is_treasury_member: bool) -> u64 {
        let (starting_balance, ending_balance) = vault_status.get_balance_of_round(round);
        return ending_balance / MemberTokenStatus::PERCENT_SHIFT
            * self.share(starting_balance, is_treasury_member);
    }
}
