// /// 
// /// Ethereum Virtual Machine (EVM) Control Flow Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM control flow instructions.
// /// 
// use ::ethereum::base_types::{U256, Uint};
// use super::super::super::vm::gas::{GAS_BASE, GAS_HIGH, GAS_JUMPDEST, GAS_MID, charge_gas};
// use super::super::::{Evm};
// use super::super::exceptions::{InvalidJumpDestError};
// use super::super::stack::{pop, push};
// /// 
// ///     Stop further execution of EVM code.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn stop(evm: Evm) -> Result<(), Error> {
//     // pass;
//     // pass;
//     evm.running = false;
//     evm.pc += 1;
// }


// /// 
// ///     Alter the program counter to the location specified by the top of the
// ///     stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn jump(evm: Evm) -> Result<(), Error> {
//     jump_dest = Uint(pop(evm.stack)?)?;
//     charge_gas(evm, GAS_MID)?;
//     if !(jump_dest).contains(evm.valid_jump_destinations) {
//         return Err(Error::InvalidJumpDestError);
//     }
//     evm.pc = Uint(jump_dest)?;
// }


// /// 
// ///     Alter the program counter to the specified location if and only if a
// ///     condition is true. If the condition is not true, then the program counter
// ///     would increase only by 1.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn jumpi(evm: Evm) -> Result<(), Error> {
//     jump_dest = Uint(pop(evm.stack)?)?;
//     conditional_value = pop(evm.stack)?;
//     charge_gas(evm, GAS_HIGH)?;
//     if conditional_value == 0 {
//         destination = evm.pc + 1;
//     } else if !(jump_dest).contains(evm.valid_jump_destinations) {
//         return Err(Error::InvalidJumpDestError);
//     } else {
//         destination = jump_dest;
//     }
//     evm.pc = Uint(destination)?;
// }


// /// 
// ///     Push onto the stack the value of the program counter after reaching the
// ///     current instruction and without increasing it for the next instruction.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn pc(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(evm.pc)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the amount of available gas (including the corresponding reduction
// ///     for the cost of this instruction) onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn gas_left(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(evm.gas_left)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Mark a valid destination for jumps. This is a noop, present only
// ///     to be used by `JUMP` and `JUMPI` opcodes to verify that their jump is
// ///     valid.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn jumpdest(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_JUMPDEST)?;
//     // pass;
//     evm.pc += 1;
// }


