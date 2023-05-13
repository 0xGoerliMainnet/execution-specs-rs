//! Ethereum Virtual Machine (EVM) Control Flow Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM control flow instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::vm::exceptions::EvmError;

/// Stop further execution of EVM code.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn stop(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    let _ = {};

    // OPERATION
    evm.running = false;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Alter the program counter to the location specified by the top of the
/// stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn jump(evm: &mut Evm) -> Result<()> {
    // STACK
    let jump_dest = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_MID())?;

    // OPERATION
    if !evm.valid_jump_destinations.contains(&jump_dest) {
        return Err(EvmError::InvalidJumpDest);
    }

    // PROGRAM COUNTEr
    evm.pc = usize::try_from(jump_dest).unwrap();
    Ok(())
}

/// Alter the program counter to the specified location if and only if a
/// condition is true. If the condition is not true, then the program counter
/// would increase only by 1.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn jumpi(evm: &mut Evm) -> Result<()> {
    // STACK
    let jump_dest = stack::pop(&mut evm.stack)?;
    let conditional_value = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_HIGH())?;

    // OPERATION
    let destination = if conditional_value == U256::from(0u8) {
        evm.pc + U256::from(1u8)
    } else if !evm.valid_jump_destinations.contains(&jump_dest) {
        return Err(EvmError::InvalidJumpDest);
    } else {
        jump_dest
    };

    // PROGRAM COUNTER
    evm.pc = usize::try_from(destination).unwrap();
    Ok(())
}

/// Push onto the stack the value of the program counter after reaching the
/// current instruction and without increasing it for the next instruction.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn pc(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, U256::from(evm.pc))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the amount of available gas (including the corresponding reduction
/// for the cost of this instruction) onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn gas_left(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.gas_left.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Mark a valid destination for jumps. This is a noop, present only
/// to be used by `JUMP` and `JUMPI` opcodes to verify that their jump is
/// valid.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn jumpdest(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_JUMPDEST())?;

    // OPERATION
    let _ = {};

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
