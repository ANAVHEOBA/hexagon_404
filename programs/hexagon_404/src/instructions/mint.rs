use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::parameters::*;
use crate::errors::error_codes::ErrorCode;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = config.authority == authority.key(),
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [b"token_account", authority.key().as_ref()],
        bump = token_account.bump,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MintTokens>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let token_account = &mut ctx.accounts.token_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Check if enough time has passed since last mint
    require!(
        current_time >= config.last_mint_timestamp + MINT_INTERVAL,
        ErrorCode::MintTooEarly
    );

    // Update mint timestamp
    config.last_mint_timestamp = current_time;

    // Mint new tokens
    let mint_amount = MINT_AMOUNT * 10u64.pow(TOKEN_DECIMALS as u32);
    token_account.balance = token_account.balance.checked_add(mint_amount)
        .ok_or(ErrorCode::Overflow)?;
    config.total_supply = config.total_supply.checked_add(mint_amount)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
