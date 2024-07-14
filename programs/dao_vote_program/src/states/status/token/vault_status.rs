use crate::constants::*;
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct TokenStatus {
    pub current_round: [u8; 8],
    pub last_slot_update: [u8; 8],
    pub fields: [u8; TokenStatus::LEN],
}

impl TokenStatus {
    // add 32?
    pub const LEN: usize = DISCRIMINATOR + (UNSIGNED_64 * 3) + (Field::LEN * 1000) + 8;

    pub const MAX_SLOT_RANGE: u64 = 216_000;

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
        self.last_slot_update = u64::to_be_bytes(slot % TokenStatus::MAX_SLOT_RANGE);

        let field = Field {
            round: 0,
            deposit_total: 0,
            withdraw_total: 0,
            starting_balance: 0,
            ending_balance: 0,
        };

        self.write(&field)?;

        Ok(())
    }

    pub fn get_balance_of_round(&self, round: u64) -> (u64, u64) {
        let Field {
            starting_balance,
            ending_balance,
            ..
        } = self.read(round as usize);
        return (starting_balance, ending_balance);
    }

    pub fn get_totals_of_round(&self, round: u16) -> (u64, u64) {
        let Field {
            deposit_total,
            withdraw_total,
            ..
        } = self.read(round as usize);
        return (deposit_total, withdraw_total);
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.update()?;

        let current_round = u64::from_be_bytes(self.current_round);
        let mut field = self.read(current_round as usize);
        field.deposit_total += amount;

        self.write(&field)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.update()?;

        let current_round = u64::from_be_bytes(self.current_round);
        let mut field = self.read(current_round as usize);
        field.withdraw_total += amount;

        self.write(&field)?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let slot = clock.slot;
        let last_slot_update = u64::from_be_bytes(self.last_slot_update);

        if (slot % TokenStatus::MAX_SLOT_RANGE) < last_slot_update {
            let mut current_round = u64::from_be_bytes(self.current_round);
            let Field {
                deposit_total,
                withdraw_total,
                starting_balance,
                ..
            } = self.read(current_round as usize);

            let balance = starting_balance + deposit_total - withdraw_total;

            current_round += 1;

            let field = Field {
                round: current_round,
                deposit_total: 0,
                withdraw_total: 0,
                starting_balance: balance,
                ending_balance: balance,
            };

            self.write(&field)?;
        }

        self.last_slot_update = u64::to_be_bytes(slot % TokenStatus::MAX_SLOT_RANGE);

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Field {
    pub round: u64,
    pub deposit_total: u64,
    pub withdraw_total: u64,
    pub starting_balance: u64,
    pub ending_balance: u64,
}

impl Field {
    pub const LEN: usize = UNSIGNED_64 * 5;
}

// ENDPOINTS:
//  - update
//  - deposit
//  - claim
