use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::error_codes::ErrorCode;

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(
        mut,
        seeds = [b"token_account", from.key().as_ref()],
        bump = from_account.bump
    )]
    pub from_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"token_account", to.key().as_ref()],
        bump = to_account.bump
    )]
    pub to_account: Account<'info, TokenAccount>,
    /// CHECK: Safe because we only read the pubkey
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let from_account = &mut ctx.accounts.from_account;
    let to_account = &mut ctx.accounts.to_account;
    let config = &mut ctx.accounts.config;

    // Calculate burn amount (1% of transfer amount)
    let burn_amount = amount.checked_mul(config.burn_rate as u64)
        .ok_or(ErrorCode::Overflow)?
        .checked_div(100)
        .ok_or(ErrorCode::Overflow)?;

    // Calculate actual transfer amount after burn
    let transfer_amount = amount.checked_sub(burn_amount)
        .ok_or(ErrorCode::Overflow)?;

    // Check if sender has enough balance
    require!(
        from_account.balance >= amount,
        ErrorCode::InsufficientBalance
    );

    // Update balances
    from_account.balance = from_account.balance.checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;
    to_account.balance = to_account.balance.checked_add(transfer_amount)
        .ok_or(ErrorCode::Overflow)?;

    // Update total supply (reduce by burn amount)
    config.total_supply = config.total_supply.checked_sub(burn_amount)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
