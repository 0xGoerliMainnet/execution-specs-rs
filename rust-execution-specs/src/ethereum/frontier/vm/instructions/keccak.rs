//! Ethereum Virtual Machine (EVM) Keccak Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM keccak instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::fork_types::keccak256;
use crate::ethereum::frontier::vm::memory::memory_read_bytes;
use crate::ethereum::utils::numeric::ceil32;

/// Pushes to the stack the Keccak-256 hash of a region of memory.
///
/// This also expands the memory, in case the memory is insufficient to
/// access the data's memory location.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn keccak(evm: &mut Evm) -> Result<()> {
    // STACK
    let memory_start_index = stack::pop(&mut evm.stack)?;
    let size = stack::pop(&mut evm.stack)?;

    // GAS
    let words = ceil32(size.clone()) / U256::from(32u8);
    let word_gas_cost = gas::GAS_KECCAK256_WORD() * words;
    let extend_memory = gas::calculate_gas_extend_memory(
        &evm.memory,
        [(memory_start_index.clone(), size.clone())].to_vec(),
    );
    gas::charge_gas(
        evm,
        gas::GAS_KECCAK256() + word_gas_cost + extend_memory.cost,
    )?;

    // OPERATION
    evm.memory
        .extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let data = memory_read_bytes(&evm.memory, memory_start_index, size);
    let hash = keccak256(data);
    stack::push(&mut evm.stack, U256::from_bytes_be(&hash))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
