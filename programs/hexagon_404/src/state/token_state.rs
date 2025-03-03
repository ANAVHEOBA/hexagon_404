use anchor_lang::prelude::*;

#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl TokenAccount {
    pub const LEN: usize = 32 + // owner
        8 +  // balance
        1 +  // bump
        32;  // buffer for future upgrades
}
