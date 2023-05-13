// /// 
// /// Ethereum Virtual Machine (EVM) Bitwise Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM bitwise instructions.
// /// 
// use ::ethereum::base_types::{U256};
// use super::super::::{Evm};
// use super::super::gas::{GAS_VERY_LOW, charge_gas};
// use super::super::stack::{pop, push};
// /// 
// ///     Bitwise AND operation of the top 2 elements of the stack. Pushes the
// ///     result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn bitwise_and(evm: Evm) -> Result<(), Error> {
//     x = pop(evm.stack)?;
//     y = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     push(evm.stack, x & y)?;
//     evm.pc += 1;
// }


// /// 
// ///     Bitwise OR operation of the top 2 elements of the stack. Pushes the
// ///     result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn bitwise_or(evm: Evm) -> Result<(), Error> {
//     x = pop(evm.stack)?;
//     y = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     push(evm.stack, x | y)?;
//     evm.pc += 1;
// }


// /// 
// ///     Bitwise XOR operation of the top 2 elements of the stack. Pushes the
// ///     result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn bitwise_xor(evm: Evm) -> Result<(), Error> {
//     x = pop(evm.stack)?;
//     y = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     push(evm.stack, x ^ y)?;
//     evm.pc += 1;
// }


// /// 
// ///     Bitwise NOT operation of the top element of the stack. Pushes the
// ///     result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn bitwise_not(evm: Evm) -> Result<(), Error> {
//     x = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     push(evm.stack, !(x))?;
//     evm.pc += 1;
// }


// /// 
// ///     For a word (defined by next top element of the stack), retrieve the
// ///     Nth byte (0-indexed and defined by top element of stack) from the
// ///     left (most significant) to right (least significant).
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn get_byte(evm: Evm) -> Result<(), Error> {
//     byte_index = pop(evm.stack)?;
//     word = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     if byte_index >= 32 {
//         result = U256(0)?;
//     } else {
//         extra_bytes_to_right = 31 - byte_index;
//         word = word >> extra_bytes_to_right * 8;
//         word = word & 255;
//         result = U256(word)?;
//     }
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


