pub struct PositionProposal {
    pub bump: u8,

    // reward to executing proposal
    pub bounty: u64,

    pub asset_token_mint: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,

    // DERIVED
    pub direction: Direction,
    pub current_price: u64,

    // INPUT
    // since we are defining the swap as token in / token out
    // might not be relavent to have "action type"
    // or actually should define as swap, and discard Buy | Sell
    pub action: Action,
    pub price: u64,
    pub amount: u64,

    // votes -> should be u128
    pub vote_yes: u64,
    pub vote_no: u64,
}

impl PositionProposal {
    pub fn init(
        &mut self,
        bump: u8,
        pool_state: Pubkey,
        input_token_account: Pubkey,
        output_token_account: Pubkey,
        // bounty: u64,
        // price: u64,
        amount: u64,
        // action: Action,
    ) {
        self.bump = bump;
        // self.bounty = bounty;

        self.asset_token_mint = asset_token_mint;
        self.pool_state = pool_state;
        self.input_token_account = input_token_account;
        self.output_token_account = output_token_account;

        // self.action = action;
        // self.price = price;
        self.amount = amount

        // derive the direction from current price,
        // need pull current price
        // for now will have the position to be instantly available to execute
        // so it is testable and don't have to worry about time constraint
    }
}

enum Action {
    Sell,
    Buy,
    Open,
    Close,
}

enum Direction {
    Increase,
    Decrease,
}
