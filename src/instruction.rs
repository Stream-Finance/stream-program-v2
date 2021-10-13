//! Instruction types
use solana_program::{
    program_error::ProgramError,
    program::{invoke},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use crate::{
    error::TokenError,
    state::{Escrow,TokenInitializeAccountParams, TokenTransferParams},

};
use std::convert::TryInto;
use spl_associated_token_account::{
    get_associated_token_address
};
/// Initialize stream data
pub struct ProcessInitializeStream{
    pub start_time: u64,
    pub end_time: u64,
    pub amount: u64,
}
/// Initialize token stream data
pub struct ProcessTokenStream{
    pub start_time: u64,
    pub end_time: u64,
    pub amount: u64,
}
pub struct Processwithdrawstream{
    /// Amount of funds locked
    pub amount: u64,
}
pub struct ProcessTokenWithdrawStream{
    /// Amount of funds locked
    pub amount: u64,
}
pub enum TokenInstruction {
    ProcessInitializeStream(ProcessInitializeStream),
    Processwithdrawstream(Processwithdrawstream),
    Processcancelstream ,
    ProcessTokenStream(ProcessTokenStream),
    ProcessPauseStream,
    ProcessResumeStream,
    ProcessTokenWithdrawStream(ProcessTokenWithdrawStream)
}
impl TokenInstruction {
    /// Unpacks a byte buffer into a [TokenInstruction](enum.TokenInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            // Initialize stream instruction 
            0 => {
                let (start_time, rest) = rest.split_at(8);
                let (end_time, rest) = rest.split_at(8);
                let (amount, _rest) = rest.split_at(8);
                let start_time = start_time.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                let end_time = end_time.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                let amount = amount.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                Self::ProcessInitializeStream (ProcessInitializeStream{start_time,end_time,amount})
            }
            // Withdraw stream instruction 
            1 => {
                let (amount, _rest) = rest.split_at(8);
                let amount = amount.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                Self::Processwithdrawstream (Processwithdrawstream{amount})
            }
            // Cancel stream instruction 
            2 => {
                Self:: Processcancelstream
            }
             // Initialize Token stream 
             3 => {
                let (start_time, rest) = rest.split_at(8);
                let (end_time, rest) = rest.split_at(8);
                let (amount, _rest) = rest.split_at(8);
                let start_time = start_time.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                let end_time = end_time.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                let amount = amount.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                Self::ProcessTokenStream (ProcessTokenStream{start_time,end_time,amount})
            }
            4 =>{
                Self::ProcessPauseStream
            }
            5 =>{
                Self::ProcessResumeStream
            }
            6 =>{
                let (amount, _rest) = rest.split_at(8);
                let amount = amount.try_into().ok().map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
                Self::ProcessTokenWithdrawStream (ProcessTokenWithdrawStream{amount})
            }
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }
}