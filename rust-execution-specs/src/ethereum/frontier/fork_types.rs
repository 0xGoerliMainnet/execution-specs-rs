/// 
/// # Ethereum Types
/// 
/// ## Introduction
/// 
/// Types re-used throughout the specification, which are specific to Ethereum.
/// 

use crate::ethereum::{base_types::{Uint, U256, Bytes, Bytes20, Bytes256, Bytes32, Bytes8}, rlp::{self, EncodeRlp}};

pub type Hash32 = [u8; 32];

pub type Address = Bytes20;
pub type Root = Hash32;
pub type Bloom = Bytes256;

pub const TX_BASE_COST : u64 = 21000;
pub const TX_DATA_COST_PER_NON_ZERO : u64 = 68;
pub const TX_DATA_COST_PER_ZERO : u64 = 4;

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
    pub nonce: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
    pub v: U256,
    pub r: U256,
    pub s: U256,
}


/// 
///     State associated with an address.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Account {
    pub nonce: Uint,
    pub balance: U256,
    pub code: Bytes,
}

pub fn empty_account() -> Account {
    Account::default()
}

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
#[derive(Clone)]
pub struct Header {
    pub parent_hash: Hash32,
    pub ommers_hash: Hash32,
    pub coinbase: Address,
    pub state_root: Root,
    pub transactions_root: Root,
    pub receipt_root: Root,
    pub bloom: Bloom,
    pub difficulty: Uint,
    pub number: Uint,
    pub gas_limit: Uint,
    pub gas_used: Uint,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub mix_digest: Bytes32,
    pub nonce: Bytes8,
}


impl Header {
}

impl EncodeRlp for Header {
    fn encode(&self) -> Bytes {
        rlp::encode(&(
            self.parent_hash,
            self.ommers_hash,
            self.coinbase,
            self.state_root,
            self.transactions_root,
            self.receipt_root,
            self.bloom,
            self.difficulty.clone(),
            self.number.clone(),
            self.gas_limit.clone(),
            self.gas_used.clone(),
            self.timestamp.clone(),
            self.extra_data.clone(),
            self.mix_digest,
            self.nonce,
        ))
    }
}


/// 
///     A complete block.
///     
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub ommers: Vec<Header>,
}


impl Block {
}


/// 
///     Data record produced during the execution of a transaction.
///     
#[derive(Clone)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<Hash32>,
    pub data: Bytes,
}


impl Log {
}


/// 
///     Result of a transaction.
///     
pub struct Receipt {
    pub post_state: Root,
    pub cumulative_gas_used: Uint,
    pub bloom: Bloom,
    pub logs: Vec<Log>,
}


impl Receipt {
}
