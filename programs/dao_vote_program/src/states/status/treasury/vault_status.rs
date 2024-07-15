use crate::constants::*;
use anchor_lang::prelude::*;

pub const TREASURY_STATUS_SEED: &str = "treasury-status";

#[account(zero_copy)]
pub struct TreasuryStatus {
    pub current_round: [u8; 8],
    pub last_slot_update: [u8; 8],
    pub fields: [u8; TreasuryStatus::LEN - BYTE * 24],
}

impl TreasuryStatus {
    pub const LEN: usize = 10240 * 10;
    pub const MAX_SLOT_RANGE: u64 = 216_000 * 30;

    pub fn next_index(&self) -> u64 {
        let total = u64::from_be_bytes(self.current_round);
        let index = total * Field::LEN as u64;
        return index;
    }

    pub fn write(&mut self, field: &Field) -> Result<()> {
        let index = field.round as usize;
        let data = field.try_to_vec()?;
        self.fields[index..(index + Field::LEN)].copy_from_slice(&data);
        Ok(())
    }

    pub fn read(&self, index: usize) -> Field {
        let data = &self.fields[index..(index + Field::LEN)];
        Field::try_from_slice(data).unwrap()
    }

    pub fn init(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let slot = clock.slot;

        self.current_round = u64::to_be_bytes(0);
        self.last_slot_update = u64::to_be_bytes(slot % TreasuryStatus::MAX_SLOT_RANGE);

        let field = Field {
            round: 0,
            deposit_total: 0,
            claim_total: 0,
            starting_balance: 0,
            ending_balance: 0,
            starting_valuation: 0,
            ending_valuation: 0,
        };

        self.write(&field)?;

        Ok(())
    }

    pub fn get_valuation_of_round(&self, round: u64) -> (u64, u64) {
        let Field {
            starting_valuation,
            ending_valuation,
            ..
        } = self.read(round as usize);
        return (starting_valuation, ending_valuation);
    }

    pub fn get_totals_of_round(&self, round: u64) -> (u64, u64) {
        let Field {
            deposit_total,
            claim_total,
            ..
        } = self.read(round as usize);
        return (deposit_total, claim_total);
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // set in validations check
        // if source.needs_update_from_source() {
        //     return;
        // }

        // self.update()?;

        let current_round = u64::from_be_bytes(self.current_round);
        let mut field = self.read(current_round as usize);
        field.deposit_total += amount;

        self.write(&field)?;

        // handled by source
        // self.last_slot_update = slot % TreasuryStatus::MAX_SLOT_RANGE;
        Ok(())
    }

    pub fn claim(&mut self, amount: u64) -> Result<()> {
        // set in validations check
        // if source.needs_update_from_source() {
        //     return;
        // }

        // self.update()?;

        let current_round = u64::from_be_bytes(self.current_round);
        let mut field = self.read(current_round as usize);
        field.claim_total += amount;

        self.write(&field)?;

        // handled by source
        // self.last_slot_update = slot % TreasuryStatus::MAX_SLOT_RANGE;

        Ok(())
    }

    pub fn is_valid_launch(&self) -> bool {
        return u64::from_be_bytes(self.current_round) != 0;
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
