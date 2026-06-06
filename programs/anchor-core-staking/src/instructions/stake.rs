use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    instructions::UpdatePluginV1CpiBuilder,
    accounts::BaseCollectionV1,
    types::{Plugin, FreezeDelegate, HookablePlugin}
};
use crate::state::Config;
use crate::constants::SEED_UPDATE_AUTHORITY;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: This account is not initialized and is being used for signing purposes only
    #[account(
        seeds = [SEED_UPDATE_AUTHORITY, collection.key().as_ref()],
        bump,
    )]
    pub update_authority: UncheckedAccount<'info>,

    #[account(
        seeds = [b"config", collection.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: This is the asset to be staked
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    #[account(mut)]
    pub collection: Account<'info, BaseCollectionV1>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is the ID of the MPL Core Program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<Stake>) -> Result<()> {
    
    // In a real implementation, we would add an Attribute Plugin to store the staked_at timestamp
    // For now, we'll implement the core logic of freezing the asset
    
    let collection_key = ctx.accounts.collection.key();
    let signer_seeds = &[
        SEED_UPDATE_AUTHORITY,
        collection_key.as_ref(),
        &[ctx.bumps.update_authority],
    ];

    // Note: This is a simplified version. Real staking would involve adding 
    // an Attribute plugin to track time.
    
    UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(Some(&ctx.accounts.collection.to_account_info()))
        .payer(&ctx.accounts.owner.to_account_info())
        .authority(Some(&ctx.accounts.update_authority.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: true }))
        .invoke_signed(&[signer_seeds])?;

    Ok(())
}
