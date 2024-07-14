pub mod program_authority;
pub use program_authority::*;

pub mod status;
pub use status::*;

pub mod proposal;
pub use proposal::*;

pub mod asset;
pub use asset::*;

// pub mod raydium_amm;
// pub use raydium_amm::*;

// locking programing tokens
//  - to earn tokens from tax coming from voting
//  - to earn yield generated from treasury investments

//  - yield_vault
//  - token_vault

//  - vault_status | vault_config
//      - yield_allocation_capital
//      - token_allocation_capital

//  - member_token_vault_status
//      - token_allocation_capital
//      - initialized_time_stamp

//  - vault_config
//      - current_round
//      -

//  - vault_status
//      - current_round
//      - last_deposit_slot: i64,
//      - fields:
//          - round: u16
//          - new capital: u64
//          - removed capital: u64
//          - starting capital: u64
//          - ending capital: u64

//  - member_status
//      - last_round
//      - added capital
//      - removed capital
//      - capital -> not sure if I need this
//      - balance
//      - share -> balance / starting_capital
//      - value -> ending_capital * share

// adding capital gets 100% on yield
// members get 90% on yield
// non-members get 60% on yield
