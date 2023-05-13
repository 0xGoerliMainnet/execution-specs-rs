// /// 
// /// Ethereum Virtual Machine (EVM) Runtime Operations
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Runtime related operations used while executing EVM code.
// /// 
// use ::typing::{Set};
// use ::ethereum::base_types::{Uint};
// use super::instructions::{Ops};
// /// 
// ///     Analyze the evm code to obtain the set of valid jump destinations.
// /// 
// ///     Valid jump destinations are defined as follows:
// ///         * The jump destination is less than the length of the code.
// ///         * The jump destination should have the `JUMPDEST` opcode (0x5B).
// ///         * The jump destination shouldn't be part of the data corresponding to
// ///           `PUSH-N` opcodes.
// /// 
// ///     Note - Jump destinations are 0-indexed.
// /// 
// ///     Parameters
// ///     ----------
// ///     code :
// ///         The EVM code which is to be executed.
// /// 
// ///     Returns
// ///     -------
// ///     valid_jump_destinations: `Set[Uint]`
// ///         The set of valid jump destinations in the code.
// ///     
// pub fn get_valid_jump_destinations(code: bytes) -> Result<Set[Uint], Error> {
//     valid_jump_destinations = set()?;
//     pc = Uint(0)?;
//     while pc < len(code)? {
//         // Try 
//             current_opcode = Ops(code[pc])?;
//         if current_opcode == Ops.JUMPDEST {
//             valid_jump_destinations.add(pc)?;
//         } else if Ops.PUSH1.value <= current_opcode.value <= Ops.PUSH32.value {
//             push_data_size = current_opcode.value - Ops.PUSH1.value + 1;
//             pc += push_data_size;
//         }
//         pc += 1;
//     }
//     return Ok(valid_jump_destinations);
// }


