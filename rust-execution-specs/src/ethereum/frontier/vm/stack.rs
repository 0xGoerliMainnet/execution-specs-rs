//! Ethereum Virtual Machine (EVM) Stack
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Implementation of the stack operators for the EVM.

use super::exceptions::{EvmError, Result};
use crate::ethereum::base_types::U256;

/// Pops the top item off of `stack`.
///
/// Parameters
/// ----------
/// stack :
///     EVM stack.
///
/// Returns
/// -------
/// value : `U256`
///     The top element on the stack.
pub fn pop(stack: &mut Vec<U256>) -> Result<U256> {
    stack.pop().ok_or(EvmError::StackUnderflow)
}

pub fn push(stack: &mut Vec<U256>, value: U256) -> Result<()> {
    if stack.len() == 1024 {
        return Err(EvmError::StackOverflow);
    }
    stack.push(value);
    Ok(())
}
