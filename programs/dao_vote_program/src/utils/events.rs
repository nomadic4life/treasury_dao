use anchor_lang::prelude::*;

#[event]
pub struct TransferEvent {
    from: Pubkey,
    to: Pubkey,
    amount: u64,
}
