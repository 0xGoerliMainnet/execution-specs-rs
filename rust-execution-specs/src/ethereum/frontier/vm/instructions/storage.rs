//! Ethereum Virtual Machine (EVM) Storage Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM storage related instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::state::{get_storage, set_storage};

/// Loads to the stack, the value corresponding to a certain key from the
/// storage of the current account.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn sload(evm: &mut Evm) -> Result<()> {
    // STACK
    let key = stack::pop(&mut evm.stack)?
        .to_bytes_be()
        .try_into()
        .unwrap();

    // GAS
    gas::charge_gas(evm, gas::GAS_SLOAD())?;

    // OPERATION
    let value = get_storage(&evm.env.state, &evm.message.current_target, &key);
    stack::push(&mut evm.stack, value)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Stores a value at a certain key in the current context's storage.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn sstore(evm: &mut Evm) -> Result<()> {
    // STACK
    let key = stack::pop(&mut evm.stack)?
        .to_bytes_be()
        .try_into()
        .unwrap();
    let new_value = stack::pop(&mut evm.stack)?;

    // GAS
    let current_value = get_storage(&evm.env.state, &evm.message.current_target, &key);
    let gas_cost = if new_value != U256::from(0u8) && current_value == U256::from(0u8) {
        gas::GAS_STORAGE_SET()
    } else {
        gas::GAS_STORAGE_UPDATE()
    };
    gas::charge_gas(evm, gas_cost)?;

    // OPERATION
    if new_value == U256::from(0u8) && current_value != U256::from(0u8) {
        evm.refund_counter += gas::GAS_STORAGE_CLEAR_REFUND();
    }
    set_storage(
        &mut evm.env.state,
        evm.message.current_target,
        &key,
        new_value,
    );

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
