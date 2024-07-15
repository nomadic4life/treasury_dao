use crate::{constants::*, states::TreasuryStatus};
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

pub const MEMBER_STATUS: &str = "member-status";

#[account]
pub struct MemberTreasuryStatus {
    pub bump: u8,
    pub authority: Pubkey,
    pub launch_member: bool,
    pub last_round: Option<u64>,
    pub deposit_total: u64,
    pub claim_total: u64,
    pub valuation: u64,
}

impl MemberTreasuryStatus {
    pub const LEN: usize =
        DISCRIMINATOR + BYTE + PUBKEY_BYTES + BYTE + (BYTE + UNSIGNED_64) + UNSIGNED_64 * 3;

    const MAX: u64 = 20;
    const PERCENT_SHIFT: u64 = 100_00;
    const MEMBER_RATE: u64 = 90;
    // const NON_MEMBER_RATE: u64 = 60;

    pub fn init(&mut self, bump: u8, member: Pubkey) {
        self.bump = bump;
        self.launch_member = false;
        self.authority = member;
        self.deposit_total = 0;
        self.claim_total = 0;
        self.valuation = 0;
        self.last_round = None;
    }

    pub fn update(&mut self, treasury_status: &TreasuryStatus) {
        let target = u64::from_be_bytes(treasury_status.current_round);
        let advance = if target - self.last_round.unwrap() <= MemberTreasuryStatus::MAX {
            1
        } else {
            (target - self.last_round.unwrap()) / MemberTreasuryStatus::MAX
        };

        for _ in 0..MemberTreasuryStatus::MAX {
            let round = self.last_round.unwrap();
            if round >= target {
                self.last_round = Some(target);
                break;
            }

            self.valuation = self.value(round, treasury_status);
            self.last_round = Some(round + advance);
        }
    }

    // need to implement a process to only claim once a round
    pub fn claim(&mut self, treasury_status: &TreasuryStatus) -> u64 {
        self.update(treasury_status);

        let amount = if self.valuation * 10 / 5 >= 100_000_000 {
            self.valuation * 10 / 5
        } else {
            self.valuation
        };

        if amount <= self.claim_total {
            self.claim_total += amount;

            // this could be an issue handling deducting the vaulation this way
            // I think should compute the amount of every asset,
            // but this could work?
            // need think about it more
            self.valuation -= amount;
        }

        return amount;
    }

    pub fn deposit(&mut self, amount: u64, treasury_status: &TreasuryStatus) {
        if self.last_round.is_none() {
            let current_round = u64::from_be_bytes(treasury_status.current_round);
            self.deposit_total = amount;
            self.last_round = Some(current_round + 1);
            return;
        }

        self.update(treasury_status);
        self.deposit_total += amount;
        self.valuation += amount;
    }

    // we lose percision because using u64,
    // because of possible overflow issue
    // can get better percision using 128
    // and get ratio by n * rate / 100.00%
    pub fn share(&self, starting_valuation: u64) -> u64 {
        let mut share = self.valuation * MemberTreasuryStatus::PERCENT_SHIFT / starting_valuation;

        // want to implement a tiered system of weigted rate / ratio of share amount
        // for now this will do?
        share = share / MemberTreasuryStatus::PERCENT_SHIFT * MemberTreasuryStatus::MEMBER_RATE;

        return share;
    }

    pub fn value(&self, round: u64, treasury_status: &TreasuryStatus) -> u64 {
        let (starting_valuation, ending_valuation) = treasury_status.get_valuation_of_round(round);
        return ending_valuation / MemberTreasuryStatus::PERCENT_SHIFT
            * self.share(starting_valuation);
    }

    pub fn claim_launch_status(&mut self) {
        self.launch_member = true;
    }

    pub fn is_valid_launch_member(&self) -> bool {
        return !(self.last_round.is_some() && self.last_round.unwrap() == 1);
    }

    pub fn is_valid_member(&self) -> bool {
        return self.last_round.is_some() && self.valuation != 0;
    }
}

// ENDPOINTS:
//  - update
//  - deposit
//  - claim

// register member status
// deposit into treasury of that batch investement
// new investment batch every 30 days

// treasury

// treasury_status
//      - current_round
//      - last_deposit_slot: i64,
//      - fields:
//          - round
//          - deposit_total
//          - claim_total
//          - starting_balance
//          - ending_balance
//          - starting_valuation
//          - ending_valuation

// member_treasury_status
//      - last_round
//      - balance | value | valuation
//      - share
