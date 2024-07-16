use crate::states::{PositionProposal, ProgramAuthority};
use crate::utils::amm_instruction;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub const AUTH_SEED: &str = "vault_and_lp_mint_auth_seed";

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        constraint = position_proposal.is_valid_position().unwrap(),
    )]
    pub position_proposal: Box<Account<'info, PositionProposal>>,

    /// CHECK: pool vault and lp mint authority
    #[account(
            seeds = [
                AUTH_SEED.as_bytes(),
            ],
            bump,
        )]
    pub authority: UncheckedAccount<'info>,

    /// CHECKED: only need the pubkey to execute the swap
    pub amm_config: UncheckedAccount<'info>,

    /// CHECKED: only need the pubkey to execute the swap
    pub pool_state: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = input_token_account.owner == program_authority.key(),
    )]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = output_token_account.owner == program_authority.key(),
    )]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub input_token_program: Interface<'info, TokenInterface>,
    pub output_token_program: Interface<'info, TokenInterface>,

    #[account(
        address = input_vault.mint
    )]
    pub input_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        address = output_vault.mint
    )]
    pub output_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECKED: only need the pubkey to execute teh swap
    pub observation_state: UncheckedAccount<'info>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
}

// using raydium
impl<'info> Swap<'info> {
    pub fn swap(&mut self) -> Result<()> {
        // need to handle this better
        let raydium_program_id = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".as_bytes();

        let amm_program = Pubkey::new_from_array(
            raydium_program_id
                .try_into()
                .expect("slice with incorrect length"),
        );

        let instruction = amm_instruction::swap_base_in(
            &amm_program,
            self.payer.key,
            self.authority.key,
            self.amm_config.key,
            self.pool_state.key,
            &self.input_token_account.key(),
            &self.output_token_account.key(),
            &self.input_vault.key(),
            &self.output_vault.key(),
            &self.input_token_program.key(),
            &self.output_token_program.key(),
            &self.input_token_mint.key(),
            &self.output_token_mint.key(),
            self.observation_state.key,
            100,
            0,
        )
        .unwrap();

        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        invoke_signed(
            &instruction,
            &[
                self.payer.to_account_info(),
                self.authority.to_account_info(),
                self.amm_config.to_account_info(),
                self.pool_state.to_account_info(),
                self.input_token_account.to_account_info(),
                self.output_token_account.to_account_info(),
                self.input_vault.to_account_info(),
                self.output_vault.to_account_info(),
                self.input_token_program.to_account_info(),
                self.output_token_program.to_account_info(),
                self.input_token_mint.to_account_info(),
                self.output_token_mint.to_account_info(),
                self.observation_state.to_account_info(),
            ],
            signer_seeds,
        )
        .unwrap();

        Ok(())
    }
}
