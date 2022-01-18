use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use switchboard_aggregator::AggregatorAccountData;
use std::convert::TryInto;

declare_id!("AbpUUoHVrTpf1S9dbr5BAeMT5dL7YieReEwmqBNsxL9i");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn create(ctx: Context<Create>) -> ProgramResult {
        ctx.accounts.trading_account.creater = ctx.accounts.creater.key();
        ctx.accounts.trading_account.is_open = false;
        ctx.accounts.trading_account.open_price = 0.0;
        ctx.accounts.trading_account.leaverage = 0;
        ctx.accounts.trading_account.wealth = 1.0;

        // TODO: mint NFT at the same time
        
        Ok(())
    }

    pub fn open_position(ctx: Context<OpenPosition>, leverage: i8) -> ProgramResult {
        ctx.accounts.trading_account.is_open = true;
        let oracle_account = &ctx.accounts.oracle_account;
        ctx.accounts.trading_account.oracle_account = oracle_account.key();
        let feed_result = AggregatorAccountData::new(oracle_account)?.get_result()?;
        ctx.accounts.trading_account.open_price = feed_result.try_into()?;
        ctx.accounts.trading_account.leaverage = leverage;
        Ok(())
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> ProgramResult {
        ctx.accounts.trading_account.is_open = false;
        let open_price: f64 = ctx.accounts.trading_account.open_price;
        let oracle_account = &ctx.accounts.oracle_account;
        let feed_result = AggregatorAccountData::new(oracle_account)?.get_result()?;
        let close_price: f64 = feed_result.try_into()?;
        let leverage: f64 = ctx.accounts.trading_account.leaverage.into();
        let current_wealth: f64 = ctx.accounts.trading_account.wealth;
        ctx.accounts.trading_account.wealth += ((close_price - open_price) * (leverage / 10.0) * current_wealth) / open_price;

        // TODO: update metadata of NFT

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(signer)]
    pub creater: AccountInfo<'info>,
    #[account(mut)]
    pub trading_account: Account<'info, TradingAccount>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(signer)]
    pub creater: AccountInfo<'info>,
    pub oracle_account: AccountInfo<'info>,
    #[account(
        mut,
        constraint = trading_account.creater == creater.key(),
        constraint = !trading_account.is_open,
    )]
    pub trading_account: Account<'info, TradingAccount>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(signer)]
    pub creater: AccountInfo<'info>,
    pub oracle_account: AccountInfo<'info>,
    #[account(
        mut,
        constraint = trading_account.creater == creater.key(),
        constraint = trading_account.is_open,
        constraint = trading_account.oracle_account == oracle_account.key(),
    )]
    pub trading_account: Account<'info, TradingAccount>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
}

#[account]
pub struct TradingAccount {
    pub creater: Pubkey,
    pub is_open: bool,
    pub oracle_account: Pubkey,
    pub open_price: f64,
    pub leaverage: i8,
    pub wealth: f64,
}