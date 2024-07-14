use anchor_lang::prelude::*;

#[account]
pub struct AllocationTracker {
    pub seed: String,
    pub current: u64,
}

impl AllocationTracker {
    const MAX_SPACE: u64 = 10240;
    const TARGET: u64 = 10240 * 50;

    pub fn increase(&mut self) -> u64 {
        if self.current >= AllocationTracker::TARGET {
            return self.current;
        }

        self.current += AllocationTracker::MAX_SPACE;
        return self.current;
    }
}
