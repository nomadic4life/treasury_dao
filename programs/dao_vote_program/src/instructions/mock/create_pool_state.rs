// this instruction is here to make a mock of pool state, so can test core functionality easily.
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{Mint, TokenInterface},
};

pub const AUTH_SEED: &str = "vault_and_lp_mint_auth_seed";

#[derive(Accounts)]
pub struct CreatePoolState<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub creator: Signer<'info>,

    /// Which config the pool belongs to.
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// CHECK: pool vault and lp mint authority
    #[account(
            seeds = [
                AUTH_SEED.as_bytes(),
            ],
            bump,
        )]
    pub authority: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_0_mint.key().as_ref(),
            token_1_mint.key().as_ref(),
        ],
        bump,
        payer = creator,
        space = PoolState::LEN
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Token_0 mint, the key must smaller then token_1 mint.
    #[account(
        constraint = token_0_mint.key() < token_1_mint.key(),
        mint::token_program = token_0_program,
    )]
    pub token_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token_1 mint, the key must grater then token_0 mint.
    #[account(
            mint::token_program = token_1_program,
        )]
    pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Token_0 vault for the pool
    #[account(
            mut,
            seeds = [
                POOL_VAULT_SEED.as_bytes(),
                pool_state.key().as_ref(),
                token_0_mint.key().as_ref()
            ],
            bump,
        )]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault for the pool
    #[account(
            mut,
            seeds = [
                POOL_VAULT_SEED.as_bytes(),
                pool_state.key().as_ref(),
                token_1_mint.key().as_ref()
            ],
            bump,
        )]
    pub token_1_vault: UncheckedAccount<'info>,

    /// pool lp mint
    #[account(
            init,
            seeds = [
                POOL_LP_MINT_SEED.as_bytes(),
                pool_state.key().as_ref(),
            ],
            bump,
            mint::decimals = 9,
            mint::authority = authority,
            payer = creator,
            mint::token_program = token_program,
        )]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    /// an account to store oracle observations
    #[account(
            init,
            seeds = [
                OBSERVATION_SEED.as_bytes(),
                pool_state.key().as_ref(),
            ],
            bump,
            payer = creator,
            space = ObservationState::LEN
        )]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,

    /// Spl token program or token program 2022
    pub token_0_program: Interface<'info, TokenInterface>,

    /// Spl token program or token program 2022
    pub token_1_program: Interface<'info, TokenInterface>,

    /// To create a new program account
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePoolState<'info> {
    pub fn init(&mut self, bumps: &InitializeProgramBumps) -> Result<()> {
        let mut pool_state = self.pool_state.load_mut()?;
        pool_state.initialize(
            auth_bump,
            100_000_000_000,
            0,
            self.creator.key(),
            self.amm_config.key(),
            self.token_0_vault.key(),
            self.token_1_vault.key(),
            &self.token_0_mint,
            &self.token_1_mint,
            &self.lp_mint,
            self.observation_state.key(),
        );

        Ok(())
    }
}
