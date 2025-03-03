use anchor_lang::prelude::*;

// Import our modules
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

// Import instruction structs directly
use instructions::{
    initialize::*,
    mint::*,
    transfer::*,
    admin::*,
};

declare_id!("GFt6B8deNhoK1S4kmU6Ko8dztqmc2zJ3tReMXG4DNSd7");

#[program]
pub mod hexagon_404 {
    use super::*;

    // Initialize the token program
    pub fn initialize(
        ctx: Context<Initialize>,
        token_name: String,
        token_symbol: String,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, token_name, token_symbol)
    }

    // Mint new tokens
    pub fn mint_tokens(ctx: Context<MintTokens>) -> Result<()> {
        instructions::mint::handler(ctx)
    }

    // Transfer tokens with burn mechanism
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        instructions::transfer::handler(ctx, amount)
    }

    // Admin functions
    pub fn update_burn_rate(ctx: Context<UpdateConfig>, new_burn_rate: u8) -> Result<()> {
        instructions::admin::update_burn_rate(ctx, new_burn_rate)
    }

    pub fn transfer_authority(ctx: Context<TransferAuthority>) -> Result<()> {
        instructions::admin::transfer_authority(ctx)
    }
}
