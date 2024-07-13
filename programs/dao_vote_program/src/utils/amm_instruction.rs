//! Instruction types

use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseIn {
    pub discriminator: [u8; 8],
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseOut {
    pub discriminator: [u8; 8],
    pub max_amount_in: u64,
    pub amount_out: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AmmInstruction {
    SwapBaseIn(SwapInstructionBaseIn),

    SwapBaseOut(SwapInstructionBaseOut),
}

impl AmmInstruction {
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::SwapBaseIn(SwapInstructionBaseIn {
                discriminator,
                amount_in,
                minimum_amount_out,
            }) => {
                buf.extend_from_slice(discriminator);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::SwapBaseOut(SwapInstructionBaseOut {
                discriminator,
                max_amount_in,
                amount_out,
            }) => {
                buf.extend_from_slice(discriminator);
                buf.extend_from_slice(&max_amount_in.to_le_bytes());
                buf.extend_from_slice(&amount_out.to_le_bytes());
            }
        }
        Ok(buf)
    }
}

pub fn get_hash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );

    return sighash;
}

pub fn swap_base_in(
    amm_program: &Pubkey,

    payer: &Pubkey,
    authority: &Pubkey,
    amm_config: &Pubkey,
    pool_state: &Pubkey,

    input_token_account: &Pubkey,
    output_token_account: &Pubkey,

    input_vault: &Pubkey,
    output_vault: &Pubkey,

    input_token_program: &Pubkey,
    output_token_program: &Pubkey,

    input_token_mint: &Pubkey,
    output_token_mint: &Pubkey,
    observation_state: &Pubkey,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let discriminator = get_hash("instructions", "swapBaseInput");
    let data = AmmInstruction::SwapBaseIn(SwapInstructionBaseIn {
        discriminator,
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*authority, false),
        AccountMeta::new_readonly(*amm_config, false),
        AccountMeta::new(*pool_state, true),
        AccountMeta::new(*input_token_account, false),
        AccountMeta::new(*output_token_account, false),
        AccountMeta::new(*input_vault, false),
        AccountMeta::new(*output_vault, false),
        AccountMeta::new(*input_token_program, false),
        AccountMeta::new(*output_token_program, false),
        AccountMeta::new(*input_token_mint, false),
        AccountMeta::new(*output_token_mint, false),
        AccountMeta::new(*observation_state, false),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}
