use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::error_codes::ErrorCode;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = config.authority == authority.key(),
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct TransferAuthority<'info> {
    #[account(mut)]
    pub current_authority: Signer<'info>,
    /// CHECK: Safe because we only read the pubkey
    pub new_authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = config.authority == current_authority.key(),
    )]
    pub config: Account<'info, Config>,
}

pub fn update_burn_rate(ctx: Context<UpdateConfig>, new_burn_rate: u8) -> Result<()> {
    require!(
        new_burn_rate <= 100,
        ErrorCode::InvalidBurnRate
    );

    let config = &mut ctx.accounts.config;
    config.burn_rate = new_burn_rate;

    emit!(BurnRateUpdated {
        old_rate: config.burn_rate,
        new_rate: new_burn_rate
    });

    Ok(())
}

pub fn transfer_authority(ctx: Context<TransferAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    emit!(AuthorityTransferred {
        old_authority: config.authority,
        new_authority: ctx.accounts.new_authority.key(),
    });

    config.authority = ctx.accounts.new_authority.key();
    Ok(())
}

// Events
#[event]
pub struct BurnRateUpdated {
    old_rate: u8,
    new_rate: u8,
}

#[event]
pub struct AuthorityTransferred {
    old_authority: Pubkey,
    new_authority: Pubkey,
}
