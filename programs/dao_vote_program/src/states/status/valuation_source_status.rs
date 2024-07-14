pub struct ValuationSourceStatus {
    pub number_of_assets: u32,
    pub up_to_date_sources: u32,
    pub last_slot_range: u64,
    // USDC valuation
    pub valuation: u64,
}

impl ValuationSourceStatus {
    pub const LEN: usize = 0;

    pub fn pulled_from_source(&mut self) {
        let clock = Clock::get()?;
        let slot = clock.slot;

        self.up_to_date_sources = 0;
        self.last_slot_update = slot % ValuationSourceStatus::MAX_SLOT_RANGE;
    }

    pub fn update_slot(&mut self) {
        let clock = Clock::get()?;
        let slot = clock.slot;

        if (slot % ValuationSourceStatus::MAX_SLOT_RANGE) < self.last_slot_range {
            self.needs_update = true;
            return;
        }

        self.last_slot_update = slot % ValuationSourceStatus::MAX_SLOT_RANGE;
    }

    pub fn is_up_to_date(&self) -> bool {
        return self.up_to_date_sources == self.number_of_assets;
    }

    pub fn needs_udpate_from_source(&mut self) -> bool {}
}

// accepted proposal
// first create the asset tracker and asset vault, should do this before / the proposal set up
// make swap
