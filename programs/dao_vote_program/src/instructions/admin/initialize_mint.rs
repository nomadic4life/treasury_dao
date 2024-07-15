use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,
    // #[account(
    //     init,
    //     payer = payer,
    //     seeds = [
    //         program_authority.key().as_ref(),
    //         b"dao-token-mint",
    //     ],
    //     bump,
    //     mint::authority = program_authority,
    //     mint::decimals = 9,
    //     mint::freeze_authority = program_authority,
    // )]
    // pub token_mint: InterfaceAccount<'info, Mint>,

    // pub token_program: Interface<'info, TokenInterface>,
    // pub system_program: Program<'info, System>,
}

impl<'info> InitializeMint<'info> {
    const MAX_SUPPLY: u64 = 10_000_000_000__000_000;

    pub fn init(&mut self) -> Result<()> {
        // self.program_authority.token_mint = self.token_mint.key();
        self.program_authority.max_supply = InitializeMint::MAX_SUPPLY;

        Ok(())
    }
}
