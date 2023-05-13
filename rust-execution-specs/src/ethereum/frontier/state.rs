// /// 
// /// State
// /// ^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// The state contains all information that is preserved between transactions.
// /// 
// /// It consists of a main account trie and storage tries for each contract.
// /// 
// /// There is a distinction between an account that does not exist and
// /// `EMPTY_ACCOUNT`.
// /// 

// // use ::dataclasses::{dataclass, field};
// // use ::typing::{Callable, Dict, List, Optional, Tuple};
// // use ::ethereum::base_types::{U256, Bytes, Uint, modify};
// // use ::ethereum::utils::ensure::{ensure};
// // use super::fork_types::{EMPTY_ACCOUNT, Account, Address, Root};
// // use super::trie::{EMPTY_TRIE_ROOT, Trie, copy_trie, root, trie_get, trie_set};

// /// 
// ///     Contains all information that is preserved between transactions.
// ///     
// struct State {
//     main_trie: Trie<Address, Option<Account>>,
//     storage_tries: HashMap<Address, Trie<Bytes, U256>>,
//     snapshots: Vec<(Trie<Address, Option<Account>>, HashMap<Address, Trie<Bytes, U256>>)>,
// }


// impl State {
// }


// /// 
// ///     Start a state transaction.
// /// 
// ///     Transactions are entirely implicit and can be nested. It is not possible to
// ///     calculate the state root during a transaction.
// /// 
// ///     Parameters
// ///     ----------
// ///     state : State
// ///         The state.
// ///     
// pub fn begin_transaction(state: &mut State) {
//     state.snapshots.append((state.main_trie.clone(), ));
// }


// /// 
// ///     Commit a state transaction.
// /// 
// ///     Parameters
// ///     ----------
// ///     state : State
// ///         The state.
// ///     
// pub fn commit_transaction(state: &mut State) {
//     state.snapshots.pop().unwrap();
// }


// /// 
// ///     Rollback a state transaction, resetting the state to the point when the
// ///     corresponding `start_transaction()` call was made.
// /// 
// ///     Parameters
// ///     ----------
// ///     state : State
// ///         The state.
// ///     
// pub fn rollback_transaction(state: State) {
//     (state._main_trie, state._storage_tries) = state._snapshots.pop()?;
// }


// /// 
// ///     Get the `Account` object at an address. Returns `EMPTY_ACCOUNT` if there
// ///     is no account at the address.
// /// 
// ///     Use `get_account_optional()` if you care about the difference between a
// ///     non-existent account and `EMPTY_ACCOUNT`.
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address to lookup.
// /// 
// ///     Returns
// ///     -------
// ///     account : `Account`
// ///         Account at address.
// ///     
// pub fn get_account(state: State, address: Address) -> Result<Account, Error> {
//     account = get_account_optional(state, address)?;
//     if isinstance(account, Account)? {
//         return Ok(account);
//     } else {
//         return Ok(EMPTY_ACCOUNT);
//     }
// }


// /// 
// ///     Get the `Account` object at an address. Returns `None` (rather than
// ///     `EMPTY_ACCOUNT`) if there is no account at the address.
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address to lookup.
// /// 
// ///     Returns
// ///     -------
// ///     account : `Account`
// ///         Account at address.
// ///     
// pub fn get_account_optional(state: State, address: Address) -> Result<Option<Account>, Error> {
//     account = trie_get(state._main_trie, address)?;
//     return Ok(account);
// }


// /// 
// ///     Set the `Account` object at an address. Setting to `None` deletes
// ///     the account (but not its storage, see `destroy_account()`).
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address to set.
// ///     account : `Account`
// ///         Account to set at address.
// ///     
// pub fn set_account(state: State, address: Address, account: Option<Account>) {
//     trie_set(state._main_trie, address, account)?;
// }


// /// 
// ///     Completely remove the account at `address` and all of its storage.
// /// 
// ///     This function is made available exclusively for the `SELFDESTRUCT`
// ///     opcode. It is expected that `SELFDESTRUCT` will be disabled in a future
// ///     hardfork and this function will be removed.
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address of account to destroy.
// ///     
// pub fn destroy_account(state: State, address: Address) {
//     destroy_storage(state, address)?;
//     set_account(state, address, ())?;
// }


// /// 
// ///     Completely remove the storage at `address`.
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address of account whose storage is to be deleted.
// ///     
// pub fn destroy_storage(state: State, address: Address) {
//     if (address).contains(state._storage_tries) {
//         // del [Subscript(Attribute(Name("state"), "_storage_tries"), [Simple(Name("address"))])];
//     }
// }


// /// 
// ///     Get a value at a storage key on an account. Returns `U256(0)` if the
// ///     storage key has not been set previously.
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address of the account.
// ///     key : `Bytes`
// ///         Key to lookup.
// /// 
// ///     Returns
// ///     -------
// ///     value : `U256`
// ///         Value at the key.
// ///     
// pub fn get_storage(state: State, address: Address, key: Bytes) -> Result<U256, Error> {
//     trie = state._storage_tries.get(address)?;
//     if (trie).is(()) {
//         return Ok(U256(0)?);
//     }
//     value = trie_get(trie, key)?;
//     assert!(isinstance(value, U256)?);
//     return Ok(value);
// }


// /// 
// ///     Set a value at a storage key on an account. Setting to `U256(0)` deletes
// ///     the key.
// /// 
// ///     Parameters
// ///     ----------
// ///     state: `State`
// ///         The state
// ///     address : `Address`
// ///         Address of the account.
// ///     key : `Bytes`
// ///         Key to set.
// ///     value : `U256`
// ///         Value to set at the key.
// ///     
// pub fn set_storage(state: State, address: Address, key: Bytes, value: U256) {
//     assert!(!(trie_get(state._main_trie, address)?).is(()));
//     trie = state._storage_tries.get(address)?;
//     if (trie).is(()) {
//         trie = Trie(secured = true, default = U256(0)?)?;
//         state._storage_tries[address] = trie;
//     }
//     trie_set(trie, key, value)?;
//     if trie._data == /* DictLiteral unsupported */ {
//         // del [Subscript(Attribute(Name("state"), "_storage_tries"), [Simple(Name("address"))])];
//     }
// }


// /// 
// ///     Calculate the storage root of an account.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The state
// ///     address :
// ///         Address of the account.
// /// 
// ///     Returns
// ///     -------
// ///     root : `Root`
// ///         Storage root of the account.
// ///     
// pub fn storage_root(state: State, address: Address) -> Result<Root, Error> {
//     assert!(state._snapshots == []);
//     if (address).contains(state._storage_tries) {
//         return Ok(root(state._storage_tries[address])?);
//     } else {
//         return Ok(EMPTY_TRIE_ROOT);
//     }
// }


// /// 
// ///     Calculate the state root.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The current state.
// /// 
// ///     Returns
// ///     -------
// ///     root : `Root`
// ///         The state root.
// ///     
// pub fn state_root(state: State) -> Result<Root, Error> {
//     assert!(state._snapshots == []);
//     pub fn get_storage_root(address: Address) -> Result<Root, Error> {
//         return Ok(storage_root(state, address)?);
//     }


//     return Ok(root(state._main_trie, get_storage_root = get_storage_root)?);
// }


// /// 
// ///     Checks if an account exists in the state trie
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The state
// ///     address:
// ///         Address of the account that needs to be checked.
// /// 
// ///     Returns
// ///     -------
// ///     account_exists : `bool`
// ///         True if account exists in the state trie, False otherwise
// ///     
// pub fn account_exists(state: State, address: Address) -> Result<bool, Error> {
//     return Ok(!(get_account_optional(state, address)?).is(()));
// }


// /// 
// ///     Checks if an account has non zero nonce or non empty code
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The state
// ///     address:
// ///         Address of the account that needs to be checked.
// /// 
// ///     Returns
// ///     -------
// ///     has_code_or_nonce : `bool`
// ///         True if if an account has non zero nonce or non empty code,
// ///         False otherwise.
// ///     
// pub fn account_has_code_or_nonce(state: State, address: Address) -> Result<bool, Error> {
//     account = get_account(state, address)?;
//     return Ok(account.nonce != Uint(0)? || account.code != []);
// }


// /// 
// ///     Modify an `Account` in the `State`.
// ///     
// pub fn modify_state(state: State, address: Address, f: Callable[[Account]][()]) {
//     set_account(state, address, modify(get_account(state, address)?, f)?)?;
// }


// /// 
// ///     Move funds between accounts.
// ///     
// pub fn move_ether(state: State, sender_address: Address, recipient_address: Address, amount: U256) {
//     pub fn reduce_sender_balance(sender: Account) {
//         ensure(sender.balance >= amount, AssertionError)?;
//         sender.balance -= amount;
//     }


//     pub fn increase_recipient_balance(recipient: Account) {
//         recipient.balance += amount;
//     }


//     modify_state(state, sender_address, reduce_sender_balance)?;
//     modify_state(state, recipient_address, increase_recipient_balance)?;
// }


// /// 
// ///     Sets the balance of an account.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The current state.
// /// 
// ///     address:
// ///         Address of the account whose nonce needs to be incremented.
// /// 
// ///     amount:
// ///         The amount that needs to set in balance.
// ///     
// pub fn set_account_balance(state: State, address: Address, amount: U256) {
//     pub fn set_balance(account: Account) {
//         account.balance = amount;
//     }


//     modify_state(state, address, set_balance)?;
// }


// /// 
// ///     Initializes an account to state.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The current state.
// /// 
// ///     address:
// ///         The address of the account that need to initialised.
// ///     
// pub fn touch_account(state: State, address: Address) {
//     if !(account_exists(state, address)?) {
//         set_account(state, address, EMPTY_ACCOUNT)?;
//     }
// }


// /// 
// ///     Increments the nonce of an account.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The current state.
// /// 
// ///     address:
// ///         Address of the account whose nonce needs to be incremented.
// ///     
// pub fn increment_nonce(state: State, address: Address) {
//     pub fn increase_nonce(sender: Account) {
//         sender.nonce += 1;
//     }


//     modify_state(state, address, increase_nonce)?;
// }


// /// 
// ///     Sets Account code.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The current state.
// /// 
// ///     address:
// ///         Address of the account whose code needs to be update.
// /// 
// ///     code:
// ///         The bytecode that needs to be set.
// ///     
// pub fn set_code(state: State, address: Address, code: Bytes) {
//     pub fn write_code(sender: Account) {
//         sender.code = code;
//     }


//     modify_state(state, address, write_code)?;
// }


// /// 
// ///     Add newly created ether to an account.
// /// 
// ///     Parameters
// ///     ----------
// ///     state:
// ///         The current state.
// ///     address:
// ///         Address of the account to which ether is added.
// ///     amount:
// ///         The amount of ether to be added to the account of interest.
// ///     
// pub fn create_ether(state: State, address: Address, amount: U256) {
//     pub fn increase_balance(account: Account) {
//         account.balance += amount;
//     }


//     modify_state(state, address, increase_balance)?;
// }


