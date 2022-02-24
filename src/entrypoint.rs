/*
the required CRATES* are brought into scope using USE*
*A crate is a binary or library.
*Use brings a path into a scope once and then call 
the items in that path as if they’re local items
*/
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
/*
entrypoint! is a MACRO* that allows us to call a program,
all calls go through the function declared as entrypoint
*macros are a way of writing code that writes other code(println!)

*/
use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data);
/*
the return value of a function is synonymous with the value
of the final expression in the block of the body of a function
RETURN is used to return early while specifying the value
*/
    Ok(())
}
