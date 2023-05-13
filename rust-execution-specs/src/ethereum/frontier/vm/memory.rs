//! Ethereum Virtual Machine (EVM) Memory
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! EVM memory operations.

use crate::ethereum::{
    base_types::{Bytes, U256},
    utils::byte::right_pad_zero_bytes,
};

/// Writes to memory.
///
/// Parameters
/// ----------
/// memory :
///     Memory contents of the EVM.
/// start_position :
///     Starting pointer to the memory.
/// value :
///     Data to write to memory.
pub fn memory_write(memory: &mut Vec<u8>, start_position: U256, value: Bytes) {
    let start_position = usize::try_from(start_position).unwrap();

    if memory.len() < start_position + value.len() {
        let missing = start_position + value.len() - memory.len();
        memory.extend(std::iter::repeat(0).take(missing));
    }

    memory[start_position..(start_position + value.len())].copy_from_slice(&value);
}

/// Read bytes from memory.
///
/// Parameters
/// ----------
/// memory :
///     Memory contents of the EVM.
/// start_position :
///     Starting pointer to the memory.
/// size :
///     Size of the data that needs to be read from `start_position`.
///
/// Returns
/// -------
/// data_bytes :
///     Data read from memory.
pub fn memory_read_bytes(memory: &[u8], start_position: U256, size: U256) -> &[u8] {
    let start_position = usize::try_from(start_position).unwrap();
    let size = usize::try_from(size).unwrap();
    &memory[start_position..(start_position + size)]
}

/// Read bytes from a buffer. Padding with zeros if neccesary.
///
/// Parameters
/// ----------
/// buffer :
///     Memory contents of the EVM.
/// start_position :
///     Starting pointer to the memory.
/// size :
///     Size of the data that needs to be read from `start_position`.
///
/// Returns
/// -------
/// data_bytes :
///     Data read from memory.
pub fn buffer_read(buffer: Bytes, start_position: U256, size: U256) -> Bytes {
    let start_position = usize::try_from(start_position).unwrap();
    let size = usize::try_from(size).unwrap();

    right_pad_zero_bytes(
        buffer[start_position..(start_position + size)]
            .to_vec()
            .into_boxed_slice(),
        size.into(),
    )
}
