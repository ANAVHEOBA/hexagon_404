use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Config {
    pub authority: Pubkey,
    pub token_name: String,
    pub token_symbol: String,
    pub total_supply: u64,
    pub decimals: u8,
    pub burn_rate: u8,  // Percentage (1 = 1%)
    pub last_mint_timestamp: i64,
    pub initialized: bool,
    // Add buffer for future upgrades
    pub bump: u8,
}

impl Config {
    pub const LEN: usize = 32 + // authority
        32 + // token_name (max)
        8 + // token_symbol (max)
        8 + // total_supply
        1 + // decimals
        1 + // burn_rate
        8 + // last_mint_timestamp
        1 + // initialized
        1 + // bump
        64; // buffer
}
