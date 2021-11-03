//! Program instruction processor

use gemachain_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, log::gema_log_slice, pubkey::Pubkey,
};

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let vec = vec![42_u8; 5];
    gema_log_slice(&vec);
    Ok(())
}
