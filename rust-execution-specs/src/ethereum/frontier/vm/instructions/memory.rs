// /// 
// /// Ethereum Virtual Machine (EVM) Memory Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM Memory instructions.
// /// 
// use ::ethereum::base_types::{U8_MAX_VALUE, U256, Bytes};
// use super::super::::{Evm};
// use super::super::gas::{GAS_BASE, GAS_VERY_LOW, calculate_gas_extend_memory, charge_gas};
// use super::super::memory::{memory_read_bytes, memory_write};
// use super::super::stack::{pop, push};
// /// 
// ///     Stores a word to memory.
// ///     This also expands the memory, if the memory is
// ///     insufficient to store the word.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn mstore(evm: Evm) -> Result<(), Error> {
//     start_position = pop(evm.stack)?;
//     value = pop(evm.stack)?.to_be_bytes32()?;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(start_position, U256(len(value)?)?)])?;
//     charge_gas(evm, GAS_VERY_LOW + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     memory_write(evm.memory, start_position, value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Stores a byte to memory.
// ///     This also expands the memory, if the memory is
// ///     insufficient to store the word.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn mstore8(evm: Evm) -> Result<(), Error> {
//     start_position = pop(evm.stack)?;
//     value = pop(evm.stack)?;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(start_position, U256(1)?)])?;
//     charge_gas(evm, GAS_VERY_LOW + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     normalized_bytes_value = Bytes([value & U8_MAX_VALUE])?;
//     memory_write(evm.memory, start_position, normalized_bytes_value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Load word from memory.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn mload(evm: Evm) -> Result<(), Error> {
//     start_position = pop(evm.stack)?;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(start_position, U256(32)?)])?;
//     charge_gas(evm, GAS_VERY_LOW + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     value = U256.from_be_bytes(memory_read_bytes(evm.memory, start_position, U256(32)?)?)?;
//     push(evm.stack, value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the size of active memory in bytes onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn msize(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(len(evm.memory)?)?)?;
//     evm.pc += 1;
// }


