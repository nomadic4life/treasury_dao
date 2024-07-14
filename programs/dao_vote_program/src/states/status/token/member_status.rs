use crate::constants::*;
use crate::states::*;
use anchor_lang::prelude::*;

#[account]
pub struct MemberTokenStatus {
    pub authority: Pubkey,
    pub last_round: Option<u16>,
    pub balance: u64,
}

impl MemberTokenStatus {
    pub const LEN: usize = DISCRIMINATOR + PUBKEY_BYTES + (BYTE + UNSIGNED_16) + UNSIGNED_64;
    const MAX: u16 = 20;
    const PERCENT_SHIFT: u64 = 100_00;
    const MEMBER_RATE: u64 = 80;
    const NON_MEMBER_RATE: u64 = 60;

    pub fn init(&mut self, member: Pubkey) {
        self.authority = member;
        self.balance = 0;
        self.last_round = None;
    }

    // breaks after 200 years. because current_round is u16 but that is okay.
    // enough time to make a solution to fix | reset current_round
    // do to time constraints, this solution will work
    pub fn update(&mut self, vault_status: &Account<TokenStatus>, is_treasury_member: bool) {
        let target = vault_status.current_round;
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

    pub fn withdraw(
        &mut self,
        amount: u64,
        vault_status: &Account<TokenStatus>,
        is_treasury_member: bool,
    ) {
        self.update(vault_status, is_treasury_member);

        if amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn deposit(
        &mut self,
        amount: u64,
        vault_status: &Account<TokenStatus>,
        is_treasury_member: bool,
    ) {
        if self.last_round.is_none() {
            self.balance = amount;
            self.last_round = Some(vault_status.current_round + 1);
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

    // using magic numbers -> need change that
    pub fn value(
        &self,
        round: u16,
        vault_status: &Account<TokenStatus>,
        is_treasury_member: bool,
    ) -> u64 {
        let (starting_balance, ending_balance) = vault_status.get_balance_of_round(round);
        return ending_balance / MemberTokenStatus::PERCENT_SHIFT
            * self.share(starting_balance, is_treasury_member);
    }
}

// ENDPOINTS:
//  - update
//  - deposit
//  - claim
