//! Ethereum Virtual Machine (EVM) Gas
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! EVM gas constants and calculators.

use super::{
    super::state::{self, State},
    exceptions::{EvmError, Result},
    Evm,
};
use crate::ethereum::base_types::U256;
use crate::ethereum::frontier::fork_types::Address;
use crate::ethereum::{base_types::Uint, utils::numeric::ceil32};
use num_traits::CheckedSub;

macro_rules! gas {
    ($($name:ident = $num:literal;)*) => {$(
        #[allow(non_snake_case)]
        pub fn $name() -> Uint {
            let num: u8 = $num;
            Uint::from(num)
        }
    )*};
}

gas! {
    GAS_JUMPDEST = 1;
    GAS_BASE = 2;
    GAS_VERY_LOW = 3;
    GAS_SLOAD = 50;
    GAS_STORAGE_SET = 20000;
    GAS_STORAGE_UPDATE = 5000;
    GAS_STORAGE_CLEAR_REFUND = 15000;
    GAS_LOW = 5;
    GAS_MID = 8;
    GAS_HIGH = 10;
    GAS_EXPONENTIATION = 10;
    GAS_EXPONENTIATION_PER_BYTE = 10;
    GAS_MEMORY = 3;
    GAS_KECCAK256 = 30;
    GAS_KECCAK256_WORD = 6;
    GAS_COPY = 3;
    GAS_BLOCK_HASH = 20;
    GAS_EXTERNAL = 20;
    GAS_BALANCE = 20;
    GAS_LOG = 375;
    GAS_LOG_DATA = 8;
    GAS_LOG_TOPIC = 375;
    GAS_CREATE = 32000;
    GAS_CODE_DEPOSIT = 200;
    GAS_ZERO = 0;
    GAS_CALL = 40;
    GAS_NEW_ACCOUNT = 25000;
    GAS_CALL_VALUE = 9000;
    GAS_CALL_STIPEND = 2300;
    REFUND_SELF_DESTRUCT = 24000;
    GAS_ECRECOVER = 3000;
    GAS_SHA256 = 60;
    GAS_SHA256_WORD = 12;
    GAS_RIPEMD160 = 600;
    GAS_RIPEMD160_WORD = 120;
    GAS_IDENTITY = 15;
    GAS_IDENTITY_WORD = 3;
}

/// Define the parameters for memory extension in opcodes
///
/// `cost`: `ethereum.base_types.Uint`
///     The gas required to perform the extension
/// `expand_by`: `ethereum.base_types.Uint`
///     The size by which the memory will be extended
pub struct ExtendMemory {
    pub cost: Uint,
    pub expand_by: Uint,
}

/// Define the gas cost and stipend for executing the call opcodes.
///
/// `cost`: `ethereum.base_types.Uint`
///     The non-refundable portion of gas reserved for executing the
///     call opcode.
/// `stipend`: `ethereum.base_types.Uint`
///     The portion of gas available to sub-calls that is refundable
///     if not consumed
pub struct MessageCallGas {
    pub cost: Uint,
    pub stipend: Uint,
}

/// Subtracts `amount` from `evm.gas_left`.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM.
/// amount :
///     The amount of gas the current operation requires.
///
pub fn charge_gas(evm: &mut Evm, amount: Uint) -> Result<()> {
    evm.gas_left = evm
        .gas_left
        .checked_sub(&amount)
        .ok_or(EvmError::OutOfGas)?;
    Ok(())
}

/// Calculates the gas cost for allocating memory
/// to the smallest multiple of 32 bytes,
/// such that the allocated size is at least as big as the given size.
///
/// Parameters
/// ----------
/// size_in_bytes :
///     The size of the data in bytes.
///
/// Returns
/// -------
/// total_gas_cost : `ethereum.base_types.Uint`
///     The gas cost for storing data in memory.
pub fn calculate_memory_gas_cost(size_in_bytes: Uint) -> Uint {
    let size_in_words = ceil32(size_in_bytes) / Uint::from(32u8);
    let linear_cost = &size_in_words * GAS_MEMORY();
    let quadratic_cost = size_in_words.pow(2) / Uint::from(512u16);
    let total_gas_cost = linear_cost + quadratic_cost;
    total_gas_cost
}

/// Calculates the gas amount to extend memory
///
/// Parameters
/// ----------
/// memory :
///     Memory contents of the EVM.
/// extensions:
///     List of extensions to be made to the memory.
///     Consists of a tuple of start position and size.
///
/// Returns
/// -------
/// extend_memory: `ExtendMemory`
pub fn calculate_gas_extend_memory(memory: &[u8], extensions: Vec<(U256, U256)>) -> ExtendMemory {
    let mut size_to_extend = Uint::from(0u8);
    let mut to_be_paid = Uint::from(0u8);
    let mut current_size = Uint::from(memory.len());

    for (start_position, size) in extensions {
        if size == Uint::from(0u8) {
            continue;
        }

        let before_size = ceil32(current_size.clone());
        let after_size = ceil32(start_position + size);
        if after_size <= before_size {
            continue;
        }

        size_to_extend += &after_size - &before_size;
        let already_paid = calculate_memory_gas_cost(before_size);
        let total_cost = calculate_memory_gas_cost(after_size.clone());
        to_be_paid += total_cost - already_paid;

        current_size = after_size;
    }

    ExtendMemory {
        cost: to_be_paid,
        expand_by: size_to_extend,
    }
}

/// Calculates the gas amount for executing Opcodes `CALL` and `CALLCODE`.
///
/// Parameters
/// ----------
/// state :
///     The current state.
/// gas :
///     The amount of gas provided to the message-call.
/// to:
///     The address of the recipient account.
/// value:
///     The amount of `ETH` that needs to be transferred.
///
/// Returns
/// -------
/// message_call_gas: `MessageCallGas`
pub fn calculate_message_call_gas(
    state: &State,
    gas: Uint,
    to: &Address,
    value: U256,
) -> MessageCallGas {
    let create_gas_cost = if state::account_exists(state, to) {
        Uint::from(0u8)
    } else {
        GAS_NEW_ACCOUNT()
    };
    let transfer_gas_cost = if value == U256::from(0u8) {
        Uint::from(0u8)
    } else {
        GAS_CALL_VALUE()
    };
    let cost = GAS_CALL() + &gas + create_gas_cost + transfer_gas_cost;
    let stipend = if value == Uint::from(0u8) {
        gas
    } else {
        GAS_CALL_STIPEND() + gas
    };

    MessageCallGas { cost, stipend }
}
