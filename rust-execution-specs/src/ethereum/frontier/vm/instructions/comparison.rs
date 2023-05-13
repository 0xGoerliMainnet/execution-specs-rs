//! Ethereum Virtual Machine (EVM) Comparison Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM Comparison instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use num_bigint::BigInt;

/// Checks if the top element is less than the next top element. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn less_than(evm: &mut Evm) -> Result<()> {
    // STACK
    let left = stack::pop(&mut evm.stack)?;
    let right = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = U256::from((left < right) as u8);
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Signed less-than comparison.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn signed_less_than(evm: &mut Evm) -> Result<()> {
    // STACK
    // fixme: sign conversion is incorrect
    let left = U256::from(stack::pop(&mut evm.stack)?);
    let right = U256::from(stack::pop(&mut evm.stack)?);

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = U256::from((left < right) as u8);
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Checks if the top element is greater than the next top element. Pushes
/// the result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn greater_than(evm: &mut Evm) -> Result<()> {
    // STACK
    let left = stack::pop(&mut evm.stack)?;
    let right = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = U256::from((left > right) as u8);
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Signed greater-than comparison.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn signed_greater_than(evm: &mut Evm) -> Result<()> {
    // STACK
    // fixme: sign conversion is incorrect
    let left = BigInt::from(stack::pop(&mut evm.stack)?);
    let right = BigInt::from(stack::pop(&mut evm.stack)?);

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = U256::from((left > right) as u8);
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Checks if the top element is equal to the next top element. Pushes
/// the result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn equal(evm: &mut Evm) -> Result<()> {
    // STACK
    let left = stack::pop(&mut evm.stack)?;
    let right = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = U256::from((left == right) as u8);
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Checks if the top element is equal to 0. Pushes the result back on the
/// stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn is_zero(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = U256::from((x == U256::from(0u8)) as u8);
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
