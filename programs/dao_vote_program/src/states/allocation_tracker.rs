use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

use crate::constants::{BYTE, DISCRIMINATOR, UNSIGNED_16};

#[account]
pub struct AllocationTracker {
    pub status_type: StatusType,
    pub program_authority: Pubkey,
    pub target_account: Pubkey,
    pub current: u16,
    pub target: u16,
}

impl AllocationTracker {
    pub const LEN: usize =
        DISCRIMINATOR + BYTE + PUBKEY_BYTES + PUBKEY_BYTES + UNSIGNED_16 + UNSIGNED_16;
    pub const MAX_SPACE: u64 = 10240;
    // const TREASURY_STATUS: str = "streasury-status";
    // const TOKEN_STATUS: str = "token-status";

    pub fn init(&mut self, status: StatusType, target_account: Pubkey, program_authority: Pubkey) {
        self.status_type = status.clone();
        self.target_account = target_account;
        self.program_authority = program_authority;

        match status {
            StatusType::TreasuryStatus => {
                self.target = 20;
            }
            StatusType::TokenStatus => {
                self.target = 20;
            }
        };
    }

    pub fn get(status: StatusType) -> String {
        match status {
            StatusType::TreasuryStatus => String::from("treasury-status"),
            StatusType::TokenStatus => String::from("token-status"),
        }
    }

    pub fn increase(&mut self) -> u64 {
        if self.current < self.target {
            self.current += 1;
        }

        return self.current as u64 * AllocationTracker::MAX_SPACE;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum StatusType {
    TreasuryStatus,
    TokenStatus,
}
