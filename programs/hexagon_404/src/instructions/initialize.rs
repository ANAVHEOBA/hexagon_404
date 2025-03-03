use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::parameters::*;
use crate::errors::error_codes::ErrorCode;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Config::LEN
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = authority,
        space = 8 + TokenAccount::LEN,
        seeds = [b"token_account", authority.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    token_name: String,
    token_symbol: String,
) -> Result<()> {
    require!(
        token_name.len() <= MAX_TOKEN_NAME_LENGTH,
        ErrorCode::TokenNameTooLong
    );
    require!(
        token_symbol.len() <= MAX_TOKEN_SYMBOL_LENGTH,
        ErrorCode::TokenSymbolTooLong
    );

    let config = &mut ctx.accounts.config;
    let token_account = &mut ctx.accounts.token_account;
    
    // Initialize config
    config.authority = ctx.accounts.authority.key();
    config.token_name = token_name;
    config.token_symbol = token_symbol;
    config.decimals = TOKEN_DECIMALS;
    config.burn_rate = BURN_RATE;
    config.last_mint_timestamp = Clock::get()?.unix_timestamp;
    config.initialized = true;

    // Calculate initial supply with decimals
    let initial_supply = MINT_AMOUNT * 10u64.pow(TOKEN_DECIMALS as u32);
    
    // Set initial supply
    config.total_supply = initial_supply;
    
    // Initialize token account with initial supply
    token_account.owner = ctx.accounts.authority.key();
    token_account.balance = initial_supply;
    token_account.bump = ctx.bumps.token_account;

    // Emit initialization event
    emit!(TokenInitialized {
        authority: config.authority,
        initial_supply,
        token_name: config.token_name.clone(),
        token_symbol: config.token_symbol.clone(),
    });

    Ok(())
}

// Event for initialization
#[event]
pub struct TokenInitialized {
    pub authority: Pubkey,
    pub initial_supply: u64,
    pub token_name: String,
    pub token_symbol: String,
}
