use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Token name is too long")]
    TokenNameTooLong,
    #[msg("Token symbol is too long")]
    TokenSymbolTooLong,
    #[msg("Not enough time has passed for next mint")]
    MintTooEarly,
    #[msg("Insufficient balance for transfer")]
    InsufficientBalance,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Program already initialized")]
    AlreadyInitialized,
    #[msg("Invalid burn rate: must be between 0 and 100")]
    InvalidBurnRate,
}
