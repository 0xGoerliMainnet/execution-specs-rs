//! Ethereum Virtual Machine (EVM) Stack Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM stack related instructions.
#![allow(dead_code)]

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::vm::exceptions::EvmError;
use crate::ethereum::frontier::vm::memory::buffer_read;

/// Remove item from stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn pop(evm: &mut Evm) -> Result<()> {
    // STACK
    stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    let _ = {};

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Pushes a N-byte immediate onto the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
///
/// num_bytes :
///     The number of immediate bytes to be read from the code and pushed to
///     the stack.
pub fn push_n(evm: &mut Evm, num_bytes: usize) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    let data_to_push = U256::from_bytes_be(&buffer_read(
        evm.code.clone(),
        U256::from(evm.pc + 1),
        U256::from(num_bytes),
    ));
    stack::push(&mut evm.stack, data_to_push)?;

    // PROGRAM COUNTER
    evm.pc += 1 + num_bytes;
    Ok(())
}

/// Duplicate the Nth stack item (from top of the stack) to the top of stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
///
/// item_number :
///     The stack item number (0-indexed from top of stack) to be duplicated
///     to the top of stack.
pub fn dup_n(evm: &mut Evm, item_number: usize) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    if !(item_number < evm.stack.len()) {
        return Err(EvmError::StackOverflow);
    }
    let data_to_duplicate = evm.stack[evm.stack.len() - 1 - item_number].clone();
    stack::push(&mut evm.stack, data_to_duplicate)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Swap the top and the `item_number` element of the stack, where
/// the top of the stack is position zero.
///
/// If `item_number` is zero, this function does nothing (which should not be
/// possible, since there is no `SWAP0` instruction).
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
///
/// item_number :
///     The stack item number (0-indexed from top of stack) to be swapped
///     with the top of stack element.
pub fn swap_n(evm: &mut Evm, item_number: usize) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;

    // OPERATION
    if !(item_number < evm.stack.len()) {
        return Err(EvmError::StackOverflow);
    }
    let other_idx = evm.stack.len() - 1 - item_number;
    let last = evm.stack.last().unwrap().clone();
    let other = evm.stack[other_idx].clone();
    *evm.stack.last_mut().unwrap() = other;
    evm.stack[other_idx] = last;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

macro_rules! make_fn {
    ($evm:ident $($fn:ident $call:expr;)*) => {$(
        fn $fn($evm: &mut Evm) -> Result<()> {
            $call
        }
    )*};
}

make_fn! {
    evm
    push1 push_n(evm, 1);
    push2 push_n(evm, 2);
    push3 push_n(evm, 3);
    push4 push_n(evm, 4);
    push5 push_n(evm, 5);
    push6 push_n(evm, 6);
    push7 push_n(evm, 7);
    push8 push_n(evm, 8);
    push9 push_n(evm, 9);
    push10 push_n(evm, 10);
    push11 push_n(evm, 11);
    push12 push_n(evm, 12);
    push13 push_n(evm, 13);
    push14 push_n(evm, 14);
    push15 push_n(evm, 15);
    push16 push_n(evm, 16);
    push17 push_n(evm, 17);
    push18 push_n(evm, 18);
    push19 push_n(evm, 19);
    push20 push_n(evm, 20);
    push21 push_n(evm, 21);
    push22 push_n(evm, 22);
    push23 push_n(evm, 23);
    push24 push_n(evm, 24);
    push25 push_n(evm, 25);
    push26 push_n(evm, 26);
    push27 push_n(evm, 27);
    push28 push_n(evm, 28);
    push29 push_n(evm, 29);
    push30 push_n(evm, 30);
    push31 push_n(evm, 31);
    push32 push_n(evm, 32);
    dup1 dup_n(evm, 0);
    dup2 dup_n(evm, 1);
    dup3 dup_n(evm, 2);
    dup4 dup_n(evm, 3);
    dup5 dup_n(evm, 4);
    dup6 dup_n(evm, 5);
    dup7 dup_n(evm, 6);
    dup8 dup_n(evm, 7);
    dup9 dup_n(evm, 8);
    dup10 dup_n(evm, 9);
    dup11 dup_n(evm, 10);
    dup12 dup_n(evm, 11);
    dup13 dup_n(evm, 12);
    dup14 dup_n(evm, 13);
    dup15 dup_n(evm, 14);
    dup16 dup_n(evm, 15);
    swap1 swap_n(evm, 1);
    swap2 swap_n(evm, 2);
    swap3 swap_n(evm, 3);
    swap4 swap_n(evm, 4);
    swap5 swap_n(evm, 5);
    swap6 swap_n(evm, 6);
    swap7 swap_n(evm, 7);
    swap8 swap_n(evm, 8);
    swap9 swap_n(evm, 9);
    swap10 swap_n(evm, 10);
    swap11 swap_n(evm, 11);
    swap12 swap_n(evm, 12);
    swap13 swap_n(evm, 13);
    swap14 swap_n(evm, 14);
    swap15 swap_n(evm, 15);
    swap16 swap_n(evm, 16);
}
