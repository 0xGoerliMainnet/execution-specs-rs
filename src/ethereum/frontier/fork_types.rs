/// 
/// Ethereum Types
/// ^^^^^^^^^^^^^^
/// 
/// .. contents:: Table of Contents
///     :backlinks: none
///     :local:
/// 
/// Introduction
/// ------------
/// 
/// Types re-used throughout the specification, which are specific to Ethereum.
/// 
// use ::dataclasses::{dataclass};
// use ::typing::{Tuple, Union};
// use super::super::::{rlp};
// use super::super::base_types::{U256, Bytes, Bytes0, Bytes8, Bytes20, Bytes32, Bytes256, Uint, slotted_freezable};
// use super::super::crypto::hash::{Hash32, keccak256};


use crate::ethereum::{base_types::{Uint, U256, Bytes, Bytes20, Bytes256, Bytes32, Bytes8}, rlp};

type Hash32 = [u8; 32];

type Address = Bytes20;
type Root = Hash32;
type Bloom = Bytes256;

const TX_BASE_COST : u64 = 21000;
const TX_DATA_COST_PER_NON_ZERO : u64 = 68;
const TX_DATA_COST_PER_ZERO : u64 = 4;

pub fn keccak256(data: &[u8]) -> Hash32 {
    use tiny_keccak::{Hasher, Keccak};

    let mut buf = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data.as_ref());
    hasher.finalize(&mut buf);
    buf
}

/// 
///     Atomic operation performed on the block chain.
///     
pub struct Transaction {
    nonce: U256,
    gas_price: U256,
    gas: U256,
    to: Option<Address>,
    value: U256,
    data: Bytes,
    v: U256,
    r: U256,
    s: U256,
}


/// 
///     State associated with an address.
///
#[derive(Debug, Default, PartialEq)]     
pub struct Account {
    nonce: Uint,
    balance: U256,
    code: Bytes,
}


// EMPTY_ACCOUNT = Account(nonce = Uint(0)?, balance = U256(0)?, code = bytearray()?)?;

/// 
///     Encode `Account` dataclass.
/// 
///     Storage is not stored in the `Account` dataclass, so `Accounts` cannot be
///     encoded with providing a storage root.
///     
pub fn encode_account(raw_account_data: Account, storage_root: Root) -> Bytes {
    rlp::encode(&(
        raw_account_data.nonce,
        raw_account_data.balance,
        storage_root,
        keccak256(&raw_account_data.code)
    ))
}


/// 
///     Header portion of a block on the chain.
///     
pub struct Header {
    parent_hash: Hash32,
    ommers_hash: Hash32,
    coinbase: Address,
    state_root: Root,
    transactions_root: Root,
    receipt_root: Root,
    bloom: Bloom,
    difficulty: Uint,
    number: Uint,
    gas_limit: Uint,
    gas_used: Uint,
    timestamp: U256,
    extra_data: Bytes,
    mix_digest: Bytes32,
    nonce: Bytes8,
}


impl Header {
}


/// 
///     A complete block.
///     
pub struct Block {
    header: Header,
    transactions: Vec<Transaction>,
    ommers: Vec<Header>,
}


impl Block {
}


/// 
///     Data record produced during the execution of a transaction.
///     
pub struct Log {
    address: Address,
    topics: Vec<Hash32>,
    data: Bytes,
}


impl Log {
}


/// 
///     Result of a transaction.
///     
pub struct Receipt {
    post_state: Root,
    cumulative_gas_used: Uint,
    bloom: Bloom,
    logs: Vec<Log>,
}


impl Receipt {
}


