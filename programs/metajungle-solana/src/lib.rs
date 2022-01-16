use anchor_lang::prelude::*;

declare_id!("7w8vbdxXoW8ezoQRukKfJGGywxe7DkqBxyTTctGesAs9");

#[program]
pub mod metajungle_solana {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
