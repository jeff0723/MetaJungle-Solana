use anchor_lang::prelude::*;

declare_id!("3suksArS2PRpzbPQbQ9yVgty8Yx1VqPgrjnbJjPdBe5J");

#[program]
pub mod metajungle_solana {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
