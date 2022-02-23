use thiserror::Error;
//similar to Java :: instead of .
//import solana_program.program_error.ProgramError 
use solana_program::program_error::ProgramError;
// use crate::{instruction::EscrowInstruction, error::EscrowError};

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}