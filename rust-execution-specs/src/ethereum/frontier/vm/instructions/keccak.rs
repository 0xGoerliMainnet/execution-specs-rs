// /// 
// /// Ethereum Virtual Machine (EVM) Keccak Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM keccak instructions.
// /// 
// use ::ethereum::base_types::{U256, Uint};
// use ::ethereum::crypto::hash::{keccak256};
// use ::ethereum::utils::numeric::{ceil32};
// use super::super::::{Evm};
// use super::super::gas::{GAS_KECCAK256, GAS_KECCAK256_WORD, calculate_gas_extend_memory, charge_gas};
// use super::super::memory::{memory_read_bytes};
// use super::super::stack::{pop, push};
// /// 
// ///     Pushes to the stack the Keccak-256 hash of a region of memory.
// /// 
// ///     This also expands the memory, in case the memory is insufficient to
// ///     access the data's memory location.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn keccak(evm: Evm) -> Result<(), Error> {
//     memory_start_index = pop(evm.stack)?;
//     size = pop(evm.stack)?;
//     words = (ceil32(Uint(size)?)?).floordiv(32);
//     word_gas_cost = GAS_KECCAK256_WORD * words;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_index, size)])?;
//     charge_gas(evm, GAS_KECCAK256 + word_gas_cost + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     data = memory_read_bytes(evm.memory, memory_start_index, size)?;
//     hash = keccak256(data)?;
//     push(evm.stack, U256.from_be_bytes(hash)?)?;
//     evm.pc += 1;
// }


