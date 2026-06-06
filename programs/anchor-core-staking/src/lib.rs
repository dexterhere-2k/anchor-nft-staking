#![allow(unexpected_cfgs, deprecated, ambiguous_glob_reexports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GonzJKpJYcQnzdhUQAkD4v4AainQkLRN6e6YVgWfwZct");

#[program]
pub mod anchor_core_staking {
    use super::*;

    pub fn create_collection(ctx: Context<CreateCollection>, name: String, uri: String) -> Result<()> {
        create_collection::handler(ctx, name, uri)
    }

    pub fn initialize(ctx: Context<Initialize>, rewards_bps: u16, freeze_period: u16) -> Result<()> {
        initialize::handler(ctx, rewards_bps, freeze_period)
    }

    pub fn mint_asset(ctx: Context<MintAsset>, name: String, uri: String) -> Result<()> {
        mint_asset::handler(ctx, name, uri)
    }
}
