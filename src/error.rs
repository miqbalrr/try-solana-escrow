use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

use solana_program::program_error::ProgramError;

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}