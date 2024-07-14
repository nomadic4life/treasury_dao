use anchor_lang::prelude::*;

#[account]
pub struct AllocationTracker {
    pub seed: String,
    pub current: u64,
}

impl AllocationTracker {
    pub const LEN: usize = 50;
    pub const MAX_SPACE: u64 = 10240;
    pub const TARGET: u64 = 10240 * 50;

    // cutting cornings because really short on time.
    pub fn init(&mut self, seed: bool) {
        if seed {
            self.seed = String::from("treasury-status");
        } else {
            self.seed = String::from("treasury-status");
        }
    }

    // cutting cornings because really short on time.
    pub fn get(seed: bool) -> String {
        let data = if seed {
            String::from("treasury-status")
        } else {
            String::from("treasury-status")
        };

        return data;
    }

    pub fn increase(&mut self) -> u64 {
        if self.current >= AllocationTracker::TARGET {
            return self.current;
        }

        self.current += AllocationTracker::MAX_SPACE;
        return self.current;
    }
}
