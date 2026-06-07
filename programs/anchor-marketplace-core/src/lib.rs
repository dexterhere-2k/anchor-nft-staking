#![allow(unexpected_cfgs, deprecated, ambiguous_glob_reexports)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

pub use state::*;
pub use instructions::*;
declare_id!("HHBCZzMZN2eNtQCKNexgkksuWMZwn8A5afBMTvUaKzEz");

#[program]
pub mod anchor_marketplace_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        msg!("Calling initialize instruction");
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        msg!("Calling list instruction");
        ctx.accounts.create_listing(price, &ctx.bumps)
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        msg!("Calling buy instruction");
        ctx.accounts.send_sol()?;
        ctx.accounts.receive_nft()?;
        ctx.accounts.receive_rewards()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        msg!("Calling delist instruction");
        ctx.accounts.delist()
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        msg!("Calling withdraw_fees instruction");
        ctx.accounts.withdraw_fees(amount)
    }
}
