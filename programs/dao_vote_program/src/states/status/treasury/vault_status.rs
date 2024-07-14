use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct TreasuryStatus {
    pub current_round: u16,
    pub last_slot_update: u64,
    pub table: Vec<Field>,
}

impl TreasuryStatus {
    pub const LEN: usize =
        DISCRIMINATOR + UNSIGNED_16 + UNSIGNED_64 + (UNSIGNED_32 + Field::LEN * u16::MAX as usize);

    pub const MAX_SLOT_RANGE: u64 = 216_000 * 30;

    pub fn init(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let slot = clock.slot;

        self.current_round = 0;
        self.last_slot_update = slot % TreasuryStatus::MAX_SLOT_RANGE;
        self.table = Vec::<Field>::new();

        self.table.push(Field {
            round: 0,
            deposit_total: 0,
            claim_total: 0,
            starting_balance: 0,
            ending_balance: 0,
            starting_valuation: 0,
            ending_valuation: 0,
        });

        Ok(())
    }

    pub fn get_valuation_of_round(&self, round: u16) -> (u64, u64) {
        let Field {
            starting_valuation,
            ending_valuation,
            ..
        } = self.table[round as usize];
        return (starting_valuation, ending_valuation);
    }

    pub fn get_totals_of_round(&self, round: u16) -> (u64, u64) {
        let Field {
            deposit_total,
            claim_total,
            ..
        } = self.table[round as usize];
        return (deposit_total, claim_total);
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // set in validations check
        // if source.needs_update_from_source() {
        //     return;
        // }

        self.table[self.current_round as usize].deposit_total += amount;

        // handled by source
        // self.last_slot_update = slot % TreasuryStatus::MAX_SLOT_RANGE;

        Ok(())
    }

    pub fn claim(&mut self, amount: u64) -> Result<()> {
        // set in validations check
        // if source.needs_update_from_source() {
        //     return;
        // }

        self.table[self.current_round as usize].claim_total += amount;

        // handled by source
        // self.last_slot_update = slot % TreasuryStatus::MAX_SLOT_RANGE;

        Ok(())
    }

    pub fn is_valid_launch(&self) -> bool {
        return self.current_round != 0;
    }

    // next_round
    // pub fn update(&mut self, source: ValuationSourceStatus) -> Result<()> {
    //     let clock = Clock::get()?;
    //     let slot = clock.slot;

    //     if (slot % TreasuryStatus::MAX_SLOT_RANGE) < self.last_slot_update && source.is_up_to_date()
    //     {
    //         let Field {
    //             deposit_total,
    //             claim_total,
    //             starting_balance,
    //             ..
    //         } = self.table[self.current_round as usize];

    //         let balance = starting_balance + deposit_total - claim_total;

    //         self.current_round += 1;
    //         self.table.push(Field {
    //             round: self.current_round,
    //             deposit_total: 0,
    //             claim_total: 0,

    //             // need to update values from source
    //             // not sure if I need balance?
    //             starting_balance: balance,
    //             ending_balance: balance,
    //             starting_valuation: source.valuation,
    //             ending_valuation: source.valuation,
    //         });

    //         source.pulled_from_source();
    //     }

    //     // self.last_slot_update = slot % TreasuryStatus::MAX_SLOT_RANGE;

    //     Ok(())
    // }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Field {
    pub round: u16,
    pub deposit_total: u64,
    pub claim_total: u64,
    pub starting_balance: u64,
    pub ending_balance: u64,
    pub starting_valuation: u64,
    pub ending_valuation: u64,
}

impl Field {
    pub const LEN: usize = UNSIGNED_16 + UNSIGNED_64 * 6;
}
