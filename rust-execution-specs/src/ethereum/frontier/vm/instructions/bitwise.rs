//! Ethereum Virtual Machine (EVM) Bitwise Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM bitwise instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::utils::numeric::get_sign;
use num_bigint::BigInt;
use num_traits::Signed;

/// Bitwise AND operation of the top 2 elements of the stack. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn bitwise_and(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    stack::push(&mut evm.stack, x & y)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Bitwise OR operation of the top 2 elements of the stack. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn bitwise_or(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    stack::push(&mut evm.stack, x | y)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Bitwise XOR operation of the top 2 elements of the stack. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn bitwise_xor(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    stack::push(&mut evm.stack, x ^ y)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Bitwise NOT operation of the top element of the stack. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn bitwise_not(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let all_set = U256::from_bytes_be(&[0xff].repeat(x.bits() as usize * 8 + 1));
    let inverse = x ^ all_set;
    stack::push(&mut evm.stack, inverse)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// For a word (defined by next top element of the stack), retrieve the
/// Nth byte (0-indexed and defined by top element of stack) from the
/// left (most significant) to right (least significant).
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn get_byte(evm: &mut Evm) -> Result<()> {
    // STACK
    let byte_index = stack::pop(&mut evm.stack)?;
    let mut word = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let result = if byte_index >= U256::from(32u8) {
        U256::from(0u8)
    } else {
        let extra_bytes_to_right = U256::from(31u8) - byte_index;
        word >>= usize::try_from(extra_bytes_to_right * U256::from(8u8)).unwrap();
        word &= U256::from(255u8);
        word
    };
    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
