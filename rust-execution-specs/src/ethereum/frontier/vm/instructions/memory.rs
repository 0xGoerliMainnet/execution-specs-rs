//! Ethereum Virtual Machine (EVM) Memory Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! 
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//! 
//! Introduction
//! ------------
//! 
//! Implementations of the EVM Memory instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::vm::memory::{memory_read_bytes, memory_write};

/// Stores a word to memory.
/// This also expands the memory, if the memory is
/// insufficient to store the word.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn mstore(evm: &mut Evm) -> Result<()> {
    // STACK
    let start_position = stack::pop(&mut evm.stack)?;
    let value = stack::pop(&mut evm.stack)?.to_bytes_be();
    
    // GAS
    let extend_memory = gas::calculate_gas_extend_memory(&evm.memory, [(start_position.clone(), U256::from(value.len()))].to_vec());
    gas::charge_gas(evm, gas::GAS_VERY_LOW() + extend_memory.cost)?;
    
    // OPERATION
    evm.memory.extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    memory_write(&mut evm.memory, start_position, value.into_boxed_slice());
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}


/// Stores a byte to memory.
/// This also expands the memory, if the memory is
/// insufficient to store the word.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn mstore8(evm: &mut Evm) -> Result<()> {
    // STACK
    let start_position = stack::pop(&mut evm.stack)?;
    let value = stack::pop(&mut evm.stack)?;
    
    // GAS
    let extend_memory = gas::calculate_gas_extend_memory(&evm.memory, [(start_position.clone(), U256::from(1u8))].to_vec());
    gas::charge_gas(evm, gas::GAS_VERY_LOW() + extend_memory.cost)?;
    
    // OPERATION
    evm.memory.extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let normalized_bytes_value = (value & U256::from(u8::MAX)).to_bytes_be().into_boxed_slice();
    memory_write(&mut evm.memory, start_position, normalized_bytes_value);
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Load word from memory.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn mload(evm: &mut Evm) -> Result<()> {
    // STACK
    let start_position = stack::pop(&mut evm.stack)?;
    
    // GAS
    let extend_memory = gas::calculate_gas_extend_memory(&evm.memory, [(start_position.clone(), U256::from(32u8))].to_vec());
    gas::charge_gas(evm, gas::GAS_VERY_LOW() + extend_memory.cost)?;
    
    // OPERATION
    evm.memory.extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let value = U256::from_bytes_be(memory_read_bytes(&evm.memory, start_position, U256::from(32u8)));
    stack::push(&mut evm.stack, value)?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}


/// Push the size of active memory in bytes onto the stack.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn msize(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};
    
    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;
    
    // OPERATION
    stack::push(&mut evm.stack, U256::from(evm.memory.len()))?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
