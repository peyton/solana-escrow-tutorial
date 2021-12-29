use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,

    /// Escrow account not rent exempt
    #[error("Escrow account not rent exempt")]
    NotRentExempt,

    /// Taker and escrow expected amount mismatch
    #[error("Mismatch between escrow amount and taker amount")]
    ExpectedAmountMismatch,

    /// Amount overflow
    #[error("Amount overflow")]
    AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
