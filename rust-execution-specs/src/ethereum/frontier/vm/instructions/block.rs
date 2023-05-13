//! Ethereum Virtual Machine (EVM) Block Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM block instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;

/// Push the hash of one of the 256 most recent complete blocks onto the
/// stack. The block number to hash is present at the top of the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn block_hash(evm: &mut Evm) -> Result<()> {
    // STACK
    let block_number = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_BLOCK_HASH())?;

    // OPERATION
    let hash =
        if evm.env.number <= block_number || evm.env.number > &block_number + U256::from(256u16) {
            U256::from(0u8)
        } else {
            // fixme: not sure about this
            // hash = evm.env.block_hashes[-(evm.env.number - block_number)]
            let hash = evm.env.block_hashes
                [!usize::try_from(evm.env.number.clone() - block_number).unwrap()];
            U256::from_bytes_be(&hash)
        };
    stack::push(&mut evm.stack, hash)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the current block's beneficiary address (address of the block miner)
/// onto the stack.
///
/// Here the current block refers to the block in which the currently
/// executing transaction/call resides.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn coinbase(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, U256::from_bytes_be(&evm.env.coinbase))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the current block's timestamp onto the stack. Here the timestamp
/// being referred is actually the unix timestamp in seconds.
///
/// Here the current block refers to the block in which the currently
/// executing transaction/call resides.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn timestamp(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.env.time.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the current block's number onto the stack.
///
/// Here the current block refers to the block in which the currently
/// executing transaction/call resides.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn number(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.env.number.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the current block's difficulty onto the stack.
///
/// Here the current block refers to the block in which the currently
/// executing transaction/call resides.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn difficulty(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.env.difficulty.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Push the current block's gas limit onto the stack.
///
/// Here the current block refers to the block in which the currently
/// executing transaction/call resides.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn gas_limit(evm: &mut Evm) -> Result<()> {
    // STACK
    let _ = {};

    // GAS
    gas::charge_gas(evm, gas::GAS_BASE())?;

    // OPERATION
    stack::push(&mut evm.stack, evm.env.gas_limit.clone())?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
