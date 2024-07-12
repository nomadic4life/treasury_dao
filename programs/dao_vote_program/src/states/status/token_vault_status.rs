use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct TokenVaultStatus {
    pub current_round: u16,
    pub last_slot_update: u64,
    pub fields: Vec<Field>,
}

impl TokenVaultStatus {
    pub const LEN: usize =
        DISCRIMINATOR + UNSIGNED_16 + UNSIGNED_64 + (UNSIGNED_32 + Field::LEN * u16::MAX as usize);

    pub const MAX_SLOT_RANGE: u64 = 216_000;

    pub fn init(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let slot = clock.slot;

        self.current_round = 0;
        self.last_slot_update = slot % TokenVaultStatus::MAX_SLOT_RANGE;
        self.fields = Vec::<Field>::new();

        self.fields.push(Field {
            round: 0,
            deposit_total: 0,
            withdraw_total: 0,
            starting_balance: 0,
            ending_balance: 0,
        });

        Ok(())
    }

    pub fn get_balance_of_round(&self, round: u16) -> (u64, u64) {
        let Field {
            starting_balance,
            ending_balance,
            ..
        } = self.fields[round as usize];
        return (starting_balance, ending_balance);
    }

    pub fn get_totals_of_round(&self, round: u16) -> (u64, u64) {
        let Field {
            deposit_total,
            withdraw_total,
            ..
        } = self.fields[round as usize];
        return (deposit_total, withdraw_total);
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.update()?;

        self.fields[self.current_round as usize].deposit_total += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.update()?;

        self.fields[self.current_round as usize].withdraw_total += amount;

        Ok(())
    }

    // breaks after 200 years. because current_round is u16 but that is okay.
    // enough time to make a solution to fix | reset current_round
    // do to time constraints, this solution will work
    pub fn update(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let slot = clock.slot;

        if (slot % TokenVaultStatus::MAX_SLOT_RANGE) < self.last_slot_update {
            let Field {
                deposit_total,
                withdraw_total,
                starting_balance,
                ..
            } = self.fields[self.current_round as usize];

            let balance = starting_balance + deposit_total - withdraw_total;

            self.current_round += 1;
            self.fields.push(Field {
                round: self.current_round,
                deposit_total: 0,
                withdraw_total: 0,
                starting_balance: balance,
                ending_balance: balance,
            });
        }

        self.last_slot_update = slot % TokenVaultStatus::MAX_SLOT_RANGE;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Field {
    pub round: u16,
    pub deposit_total: u64,
    pub withdraw_total: u64,
    pub starting_balance: u64,
    pub ending_balance: u64,
}

impl Field {
    pub const LEN: usize = UNSIGNED_16 + UNSIGNED_64 * 4;
}

// ENDPOINTS:
//  - update
//  - deposit
//  - claim
