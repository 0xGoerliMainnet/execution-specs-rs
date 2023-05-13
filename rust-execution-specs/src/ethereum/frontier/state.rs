//!
//! State
//! ^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! The state contains all information that is preserved between transactions.
//!
//! It consists of a main account trie and storage tries for each contract.
//!
//! There is a distinction between an account that does not exist and
//! `EMPTY_ACCOUNT`.
//!

// // use ::dataclasses::{dataclass, field};
// // use ::typing::{Callable, Dict, List, Optional, Tuple};
// // use ::ethereum::base_types::{U256, Bytes, Uint, modify};
// // use ::ethereum::utils::ensure::{ensure};
// // use super::fork_types::{EMPTY_ACCOUNT, Account, Address, Root};
// // use super::trie::{EMPTY_TRIE_ROOT, Trie, copy_trie, root, trie_get, trie_set};

use super::{
    fork_types::{Account, Address, Root, empty_account},
    trie::{self, Trie},
};
use crate::ethereum::base_types::{Bytes, Uint, U256, Bytes32};
use num_traits::CheckedSub;
use std::collections::HashMap;

/// Contains all information that is preserved between transactions.
pub struct State {
    main_trie: Trie<Address, Option<Account>>,
    storage_tries: HashMap<Address, Trie<Bytes32, U256>>,
    snapshots: Vec<(
        Trie<Address, Option<Account>>,
        HashMap<Address, Trie<Bytes32, U256>>,
    )>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            main_trie: Trie::new(true, None),
            storage_tries: HashMap::new(),
            snapshots: Vec::new(),
        }
    }
}

/// Free resources held by the state. Used by optimized implementations to
/// release file descriptors.
pub fn close_state(_: State) {}

/// Start a state transaction.
///
/// Transactions are entirely implicit and can be nested. It is not possible to
/// calculate the state root during a transaction.
///
/// Parameters
/// ----------
/// state : State
///     The state.
pub fn begin_transaction(state: &mut State) {
    state
        .snapshots
        .push((state.main_trie.clone(), state.storage_tries.clone()));
}

/// Commit a state transaction.
///
/// Parameters
/// ----------
/// state : State
///     The state.
///
pub fn commit_transaction(state: &mut State) {
    state.snapshots.pop();
}

/// Rollback a state transaction, resetting the state to the point when the
/// corresponding `start_transaction()` call was made.
///
/// Parameters
/// ----------
/// state : State
///     The state.
///
pub fn rollback_transaction(state: &mut State) {
    let (main_trie, storage_tries) = state.snapshots.pop().expect("No ongoing transaction");
    state.main_trie = main_trie;
    state.storage_tries = storage_tries;
}

/// Get the `Account` object at an address. Returns `EMPTY_ACCOUNT` if there
/// is no account at the address.
///
/// Use `get_account_optional()` if you care about the difference between a
/// non-existent account and `EMPTY_ACCOUNT`.
///
/// Parameters
/// ----------
/// state: `State`
///     The state
/// address : `Address`
///     Address to lookup.
///
/// Returns
/// -------
/// account : `Account`
///     Account at address.
pub fn get_account(state: &State, address: &Address) -> Account {
    get_account_optional(state, address).unwrap_or(empty_account())
}

/// Get the `Account` object at an address. Returns `None` (rather than
/// `EMPTY_ACCOUNT`) if there is no account at the address.
///
/// Parameters
/// ----------
/// state: `State`
///     The state
/// address : `Address`
///     Address to lookup.
///
/// Returns
/// -------
/// account : `Account`
///     Account at address.
pub fn get_account_optional(state: &State, address: &Address) -> Option<Account> {
    trie::trie_get(&state.main_trie, address)
}

/// Set the `Account` object at an address. Setting to `None` deletes
/// the account (but not its storage, see `destroy_account()`).
///
/// Parameters
/// ----------
/// state: `State`
/// The state
/// address : `Address`
/// Address to set.
/// account : `Account`
/// Account to set at address.
pub fn set_account(state: &mut State, address: Address, account: Option<Account>) {
    trie::trie_set(&mut state.main_trie, address, account);
}

/// Completely remove the account at `address` and all of its storage.
///
/// This function is made available exclusively for the `SELFDESTRUCT`
/// opcode. It is expected that `SELFDESTRUCT` will be disabled in a future
/// hardfork and this function will be removed.
///
/// Parameters
/// ----------
/// state: `State`
///     The state
/// address : `Address`
///     Address of account to destroy.
///
pub fn destroy_account(state: &mut State, address: &Address) {
    destroy_storage(state, address);
    set_account(state, address.clone(), None);
}

/// Completely remove the storage at `address`.
///
/// Parameters
/// ----------
/// state: `State`
///     The state
/// address : `Address`
///     Address of account whose storage is to be deleted.
///
pub fn destroy_storage(state: &mut State, address: &Address) {
    state.storage_tries.remove(address);
}

/// Get a value at a storage key on an account. Returns `U256(0)` if the
/// storage key has not been set previously.
///
/// Parameters
/// ----------
/// state: `State`
///     The state
/// address : `Address`
///     Address of the account.
/// key : `Bytes`
///     Key to lookup.
///
/// Returns
/// -------
/// value : `U256`
///     Value at the key.
///
pub fn get_storage(state: &State, address: &Address, key: &Bytes32) -> U256 {
    let Some(trie) = state.storage_tries.get(address) else {
        return U256::from(0u8);
    };
    trie::trie_get(&trie, key)
}

/// Set a value at a storage key on an account. Setting to `U256(0)` deletes
/// the key.
///
/// Parameters
/// ----------
/// state: `State`
///     The state
/// address : `Address`
///     Address of the account.
/// key : `Bytes`
///     Key to set.
/// value : `U256`
///     Value to set at the key.
///
pub fn set_storage(state: &mut State, address: Address, key: &Bytes32, value: U256) {
    // assert!(trie::trie_get(&state.main_trie).is_some());

    let trie = state
        .storage_tries
        .entry(address)
        .or_insert_with(|| Trie::new(true, Uint::default()));
    trie::trie_set(trie, key.clone(), value);
    // todo
    // if trie._data == {}:
    //         del state._storage_tries[address]
}

/// Calculate the storage root of an account.
///
/// Parameters
/// ----------
/// state:
///     The state
/// address :
///     Address of the account.
///
/// Returns
/// -------
/// root : `Root`
///     Storage root of the account.
///
pub fn storage_root(state: &State, address: &Address) -> Root {
    assert!(state.snapshots.is_empty());
    let z = state
        .storage_tries
        .get(address)
        .map(|trie| trie::root(trie, |_| Root::default()))
        .unwrap()
        // .unwrap_or(trie::EMPTY_TRIE_ROOT.clone());
    ;
    z
}

/// Calculate the state root.
///
/// Parameters
/// ----------
/// state:
///     The current state.
///
/// Returns
/// -------
/// root : `Root`
///     The state root.
///
pub fn state_root(state: &State) -> Root {
    assert!(state.snapshots.is_empty());

    let get_state_root = |address: &Address| -> Root { storage_root(state, address) };
    trie::root(&state.main_trie, get_state_root)
}

/// Checks if an account exists in the state trie
///
/// Parameters
/// ----------
/// state:
///     The state
/// address:
///     Address of the account that needs to be checked.
///
/// Returns
/// -------
/// account_exists : `bool`
///     True if account exists in the state trie, False otherwise
///
pub fn account_exists(state: &State, address: &Address) -> bool {
    get_account_optional(state, address).is_some()
}

/// Checks if an account has non zero nonce or non empty code
///
/// Parameters
/// ----------
/// state:
///     The state
/// address:
///     Address of the account that needs to be checked.
///
/// Returns
/// -------
/// has_code_or_nonce : `bool`
///     True if if an account has non zero nonce or non empty code,
///     False otherwise.
///
pub fn account_has_code_or_nonce(state: &State, address: &Address) -> bool {
    let account = get_account(state, address);
    account.nonce != Uint::from(0u8) || *account.code != *b""
}

/// Modify an `Account` in the `State`.
pub fn modify_state<F>(state: &mut State, address: Address, f: F)
where
    F: FnOnce(&mut Account),
{
    let mut account = get_account(state, &address).clone();
    f(&mut account);
    set_account(state, address, Some(account));
}

/// Move funds between accounts.
pub fn move_ether(
    state: &mut State,
    sender_address: Address,
    recipient_address: Address,
    amount: U256,
) {
    let sub_amount = amount.clone();

    let reduce_sender_balance = |sender: &mut Account| {
        sender.balance = sender.balance.checked_sub(&sub_amount).expect("Sender does not have enough ether");
    };

    let increase_recipient_balance = |recipient: &mut Account| {
        recipient.balance += amount;
    };

    modify_state(state, sender_address, reduce_sender_balance);
    modify_state(state, recipient_address, increase_recipient_balance);
}

/// Sets the balance of an account.
///
/// Parameters
/// ----------
/// state:
///     The current state.
///
/// address:
///     Address of the account whose nonce needs to be incremented.
///
/// amount:
///     The amount that needs to set in balance.
///
pub fn set_account_balance(state: &mut State, address: Address, amount: U256) {
    let set_balance = |account: &mut Account| {
        account.balance = amount;
    };

    modify_state(state, address, set_balance)
}

/// Initializes an account to state.
///
/// Parameters
/// ----------
/// state:
///     The current state.
///
/// address:
///     The address of the account that need to initialised.
///
pub fn touch_account(state: &mut State, address: Address) {
    if !account_exists(state, &address) {
        set_account(state, address, Some(empty_account()));
    }
}

/// Increments the nonce of an account.
///
/// Parameters
/// ----------
/// state:
///     The current state.
///
/// address:
///     Address of the account whose nonce needs to be incremented.
///
pub fn increment_nonce(state: &mut State, address: Address) {
    let increase_nonce = |sender: &mut Account| {
        sender.nonce += Uint::from(1u8);
    };
    modify_state(state, address, increase_nonce);
}

/// Sets Account code.
///
/// Parameters
/// ----------
/// state:
///     The current state.
///
/// address:
///     Address of the account whose code needs to be update.
///
/// code:
///     The bytecode that needs to be set.
///
pub fn set_code(state: &mut State, address: Address, code: Bytes) {
    let write_code = |sender: &mut Account| {
        sender.code = code;
    };
    modify_state(state, address, write_code);
}

/// Add newly created ether to an account.
///
/// Parameters
/// ----------
/// state:
///     The current state.
/// address:
///     Address of the account to which ether is added.
/// amount:
///     The amount of ether to be added to the account of interest.
///
pub fn create_ether(state: &mut State, address: Address, amount: U256) {
    let increase_balance = |account: &mut Account| {
        account.balance += amount;
    };
    modify_state(state, address, increase_balance);
}
