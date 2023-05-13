use std::collections::HashSet;

use super::{
    fork_types::{Address, Hash32, Log},
    state::State,
};
use crate::ethereum::base_types::{Bytes, Uint, U256};

pub mod instructions;
pub mod exceptions;
pub mod gas;
pub mod memory;
pub mod stack;

/// Items external to the virtual machine itself, provided by the environment.
pub struct Environment {
    pub caller: Address,
    pub block_hashes: Vec<Hash32>,
    pub origin: Address,
    pub coinbase: Address,
    pub number: Uint,
    pub gas_limit: Uint,
    pub gas_price: U256,
    pub time: U256,
    pub difficulty: Uint,
    pub state: State,
}

/// Items that are used by contract creation or message call.
pub struct Message {
    pub caller: Address,
    pub target: Option<Address>,
    pub current_target: Address,
    pub gas: U256,
    pub value: U256,
    pub data: Bytes,
    pub code_address: Option<Address>,
    pub code: Bytes,
    pub depth: Uint,
}

/// The internal state of the virtual machine.
pub struct Evm {
    pc: usize,
    stack: Vec<U256>,
    memory: Vec<u8>,
    code: Bytes,
    gas_left: U256,
    env: Environment,
    valid_jump_destinations: HashSet<U256>,
    logs: Vec<Log>,
    refund_counter: U256,
    running: bool,
    message: Message,
    output: Bytes,
    accounts_to_delete: HashSet<Address>,
    has_erred: bool,
}

/// Incorporate the state of a successful `child_evm` into the parent `evm`.
///
/// Parameters
/// ----------
/// evm :
///     The parent `EVM`.
/// child_evm :
///     The child evm to incorporate.
///
pub fn incorporate_child_on_success(evm: &mut Evm, child_evm: &Evm) {
    evm.gas_left += &child_evm.gas_left;
    evm.logs.extend(child_evm.logs.clone());
    evm.refund_counter += &child_evm.refund_counter;
    evm.accounts_to_delete
        .extend(child_evm.accounts_to_delete.clone());
}

/// Incorporate the state of an unsuccessful `child_evm` into the parent `evm`.
///
/// Parameters
/// ----------
/// evm :
///     The parent `EVM`.
/// child_evm :
///     The child evm to incorporate.
///
pub fn incorporate_child_on_error(evm: &mut Evm, child_evm: &Evm) {
    evm.gas_left += &child_evm.gas_left;
}
