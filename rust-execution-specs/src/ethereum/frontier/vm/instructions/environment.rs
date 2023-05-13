//! Ethereum Virtual Machine (EVM) Environmental Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM environment related instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::state::get_account;
use crate::ethereum::frontier::vm::memory::{buffer_read, memory_write};
use crate::ethereum::utils::numeric::ceil32;

/// Pushes the address of the current executing account to the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn address(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(
        &mut evm.stack,
        U256::from_bytes_be(&evm.message.current_target),
    )?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Pushes the balance of the given account onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn balance(evm: &mut Evm) -> Result<()> {
    // STACK
    let address = stack::pop(&mut evm.stack)?
        .to_bytes_be()
        .try_into()
        .unwrap();

    // GAS
    gas::charge_gas(evm, gas::GAS_BALANCE())?;

    // OPERATION
    let balance = get_account(&evm.env.state, &address).balance;
    stack::push(&mut evm.stack, balance)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Pushes the address of the original transaction sender to the stack.
/// The origin address can only be an EOA.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn origin(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, U256::from_bytes_be(&evm.env.origin))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Pushes the address of the caller onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn caller(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, U256::from_bytes_be(&evm.message.caller))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the value (in wei) sent with the call onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn callvalue(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.message.value.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push a word (32 bytes) of the input data belonging to the current
/// environment onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn calldataload(evm: &mut Evm) -> Result<()> {
    // STACK
    let start_index = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let value = buffer_read(evm.message.data.clone(), start_index, U256::from(32u8));
    stack::push(&mut evm.stack, U256::from_bytes_be(&value))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the size of input data in current environment onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn calldatasize(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, U256::from(evm.message.data.len()))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Copy a portion of the input data in current environment to memory.
///
/// This will also expand the memory, in case that the memory is insufficient
/// to store the data.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn calldatacopy(evm: &mut Evm) -> Result<()> {
    // STACK
    let memory_start_index = stack::pop(&mut evm.stack)?;
    let data_start_index = stack::pop(&mut evm.stack)?;
    let size = stack::pop(&mut evm.stack)?;

    // GAS
    let words = ceil32(size.clone()) / U256::from(32u8);
    let copy_gas_cost = gas::GAS_COPY() * words;
    let extend_memory = gas::calculate_gas_extend_memory(
        &evm.memory,
        [(memory_start_index.clone(), size.clone())].to_vec(),
    );
    gas::charge_gas(
        evm,
        gas::GAS_VERY_LOW() + copy_gas_cost + extend_memory.cost,
    )?;

    // OPERATION
    evm.memory
        .extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let value = buffer_read(evm.message.data.clone(), data_start_index, size);
    memory_write(&mut evm.memory, memory_start_index, value);

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the size of code running in current environment onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn codesize(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, U256::from(evm.code.len()))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Copy a portion of the code in current environment to memory.
///
/// This will also expand the memory, in case that the memory is insufficient
/// to store the data.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn codecopy(evm: &mut Evm) -> Result<()> {
    // STACK
    let memory_start_index = stack::pop(&mut evm.stack)?;
    let code_start_index = stack::pop(&mut evm.stack)?;
    let size = stack::pop(&mut evm.stack)?;

    // GAS
    let words = ceil32(size.clone()) / U256::from(32u8);
    let copy_gas_cost = gas::GAS_COPY() * words;
    let extend_memory = gas::calculate_gas_extend_memory(
        &evm.memory,
        [(memory_start_index.clone(), size.clone())].to_vec(),
    );
    gas::charge_gas(
        evm,
        gas::GAS_VERY_LOW() + copy_gas_cost + extend_memory.cost,
    )?;

    // OPERATION
    evm.memory
        .extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let value = buffer_read(evm.code.clone(), code_start_index, size);
    memory_write(&mut evm.memory, memory_start_index, value);

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the gas price used in current environment onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn gasprice(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.env.gas_price.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the code size of a given account onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn extcodesize(evm: &mut Evm) -> Result<()> {
    // STACK
    let address = stack::pop(&mut evm.stack)?
        .to_bytes_be()
        .try_into()
        .unwrap();

    // GAS
    gas::charge_gas(evm, gas::GAS_EXTERNAL())?;

    // OPERATION
    let codesize = U256::from_bytes_be(&get_account(&evm.env.state, &address).code);
    stack::push(&mut evm.stack, codesize)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Copy a portion of an account's code to memory.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn extcodecopy(evm: &mut Evm) -> Result<()> {
    // STACK
    let address = stack::pop(&mut evm.stack)?
        .to_bytes_be()
        .try_into()
        .unwrap();
    let memory_start_index = stack::pop(&mut evm.stack)?;
    let code_start_index = stack::pop(&mut evm.stack)?;
    let size = stack::pop(&mut evm.stack)?;

    // GAS
    let words = ceil32(size.clone()) / U256::from(32u8);
    let copy_gas_cost = gas::GAS_COPY() * words;
    let extend_memory = gas::calculate_gas_extend_memory(
        &evm.memory,
        [(memory_start_index.clone(), size.clone())].to_vec(),
    );
    gas::charge_gas(
        evm,
        gas::GAS_EXTERNAL() + copy_gas_cost + extend_memory.cost,
    )?;

    // OPERATION
    evm.memory
        .extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let code = get_account(&evm.env.state, &address).code;
    let value = buffer_read(code, code_start_index, size);
    memory_write(&mut evm.memory, memory_start_index, value);

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
