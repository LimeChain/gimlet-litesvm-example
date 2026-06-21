#![no_std]

use pinocchio::{
    cpi::invoke,
    error::ProgramError,
    instruction::{InstructionAccount, InstructionView},
    AccountView, Address, ProgramResult,
};
use solana_program_log::log;

#[cfg(not(feature = "no-entrypoint"))]
pinocchio::program_entrypoint!(process_instruction);
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::default_allocator!();
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::nostd_panic_handler!();

pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 32 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&instruction_data[..32]);
    let cpi_target_program_id = Address::new_from_array(bytes);

    let payer = accounts.first().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let cpi_target_program = accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !payer.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    log!("Greetings from primary");
    log!("Invoking cpi_target");

    if cpi_target_program.address() != &cpi_target_program_id {
        return Err(ProgramError::InvalidArgument);
    }

    let metas = [InstructionAccount::writable_signer(payer.address())];
    let ix = InstructionView {
        program_id: cpi_target_program.address(),
        accounts: &metas,
        data: &[],
    };
    invoke(&ix, &[payer])?;

    Ok(())
}
