//! Ethereum Virtual Machine (EVM) Logging Instructions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementations of the EVM logging instructions.

use super::super::{exceptions::Result, gas, stack, Evm};
use crate::ethereum::frontier::fork_types::Log;
use crate::ethereum::frontier::vm::memory::memory_read_bytes;

/// Appends a log entry, having `num_topics` topics, to the evm logs.
///
/// This will also expand the memory if the data (required by the log entry)
/// corresponding to the memory is not accessible.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
/// num_topics :
///     The number of topics to be included in the log entry.
pub fn log_n(evm: &mut Evm, num_topics: usize) -> Result<()> {
    // STACK
    let memory_start_index = stack::pop(&mut evm.stack)?;
    let size = stack::pop(&mut evm.stack)?;

    let mut topics = Vec::new();
    for _ in 0..num_topics {
        let topic = stack::pop(&mut evm.stack)?.to_bytes_be();
        topics.push(topic.try_into().unwrap());
    }

    // GAS
    let extend_memory = gas::calculate_gas_extend_memory(
        &evm.memory,
        [(memory_start_index.clone(), size.clone())].to_vec(),
    );
    gas::charge_gas(
        evm,
        gas::GAS_LOG()
            + gas::GAS_LOG_DATA() * &size
            + gas::GAS_LOG_TOPIC() * num_topics
            + extend_memory.cost,
    )?;

    // OPERATION
    evm.memory
        .extend([0].repeat(usize::try_from(extend_memory.expand_by).unwrap()));
    let log_entry = Log {
        address: evm.message.current_target,
        topics,
        data: memory_read_bytes(&evm.memory, memory_start_index, size)
            .to_vec()
            .into_boxed_slice(),
    };
    evm.logs.push(log_entry);

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

pub fn log0(evm: &mut Evm) -> Result<()> {
    log_n(evm, 0)
}

pub fn log1(evm: &mut Evm) -> Result<()> {
    log_n(evm, 1)
}

pub fn log4(evm: &mut Evm) -> Result<()> {
    log_n(evm, 4)
}
