use anchor_lang::prelude::*;

declare_id!("9TyRZTroFNPpQNLs8q5zEZdEHGJXzTeXNtjYx9n9W5XA");

#[program]
pub mod dao_vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
