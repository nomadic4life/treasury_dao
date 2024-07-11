pub struct MemberTokenVaultStatus {
    pub bump: u8,
    pub last_round: Option<u16>,
    pub balance: u64,
    pub status: Status,
    pub is_member: bool,
}

enum Status {
    Deposit,
    Claim { pub round: u16 },
}

impl MemberTokenVaultStatus {
    pub fn init(&mut self) {
        self.balance = 0;
        self.round = None;
        self.status = Status::Deposit;
    }

    pub fn update(&mut self, amount: u64, round: u16, vault_status: VaultStatus) -> i64 {
        if last_round.is_none() {
            self.balance = amount;
            self.round = Some(round);
            self.status = Status::Deposit;
            return;
        }

        match self.status {
            Deposit => self.deposit(amount, round, vault_status),
            Claim => self.claim(amoun, round, vault_status),
        }
    }

    pub fn claim_status(&mut self, round: u16) {
        self.status = Status::Claim { round }
    }

    pub fn claim(&mut self, advance: u16, target: u16, vault_status: VaultStatus) {
        // lets try to maximize this, starting with 20 iterations
        for _ in ..20 {
            let round = self.round.unwrap();
            if round >= target {
                self.round = Some(end);
                break;
            }

            self.balance = self.value(round, vault_status);
            self.round = Some(round + advance);
        }
    }

    pub fn deposit(&mut self, advance: u16, vault_status: VaultStatus) {
        for _ in ..20 {
            let round = self.round.unwrap();
            if round >= vault_status.round {
                self.round = Some(vault_status.round);
                break;
            }

            self.balance = self.value(round, vault_status);
            self.round = Some(round + advance);
        }
    }

    pub fn share(&self, starting_capital: u64) -> u64 {
        let mut share = self.balance * 10000 / starting_capital;
        if !self.is_member {
            share = share / 100 * 60;
        } else {
            share = share / 100 * 90;
        }

        return share;
    }

    pub fn value(&self, round: u16, vault_status: VaultStatus) -> u64 {
        let (starting_captial, ending_captial) = vault_status.get_capital(round);
        ending_capital / 10000 * self.share(starting_capital);
    }
}

// TRADE OFF
//      if tracking active account, then have to iterate 1 round at a time
//      if not tracking active account, then can skip by 20 | 200 rounds at a time
//      going with not tracking so can implement skip, makes things simpler and easier, probably better
// vault_status.deduct_active_account(round);
