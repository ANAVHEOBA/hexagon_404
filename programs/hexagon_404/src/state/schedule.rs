use anchor_lang::prelude::*;

#[account]
pub struct MintSchedule {
    pub last_mint_time: i64,
    pub next_mint_time: i64,
    pub total_mints_performed: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

impl MintSchedule {
    pub const LEN: usize = 8 + // last_mint_time
        8 + // next_mint_time
        8 + // total_mints_performed
        32 + // authority
        1 + // bump
        32; // buffer for future upgrades
}

#[event]
pub struct MintScheduleUpdated {
    pub last_mint_time: i64,
    pub next_mint_time: i64,
    pub total_mints_performed: u64,
}
