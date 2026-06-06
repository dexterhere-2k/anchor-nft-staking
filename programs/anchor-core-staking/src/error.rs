use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid update authority")]
    InvalidUpdateAuthority,
    #[msg("Freeze period has not elapsed")]
    FreezePeriodNotElapsed,
}
