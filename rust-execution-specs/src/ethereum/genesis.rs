//!
//! # Genesis Configuration
//!
//! ## Introduction
//!
//! Functionalities and entities to obtain the genesis configurations for
//! different chains.
//!
use std::{collections::HashMap, todo};

use num_bigint::BigUint;

use crate::{
    ethereum::{
        frontier::{
            fork_types::{Hash32, Header},
            state::{self, state_root, State},
            trie::{root, Trie, dummy_root},
        },
        rlp,
    },
    ethereum_spec_tools::forks::HardFork,
};

use super::{
    base_types::{Bytes, Bytes20, Bytes8, Uint, U256, U64},
    exceptions::EthereumException,
    frontier::fork::BlockChain,
    utils::hexadecimal::{hex_to_bytes, hex_to_bytes20, hex_to_bytes8, hex_to_u256, hex_to_uint},
};

type Address = Bytes20;

///
///     Configuration for the first block of an Ethereum chain.
///
///     Specifies the allocation of ether set out in the pre-sale, and some of
///     the fields of the genesis block.
///
#[derive(Default)]
pub struct GenesisConfiguration {
    pub chain_id: U64,
    pub difficulty: Uint,
    pub extra_data: Bytes,
    pub gas_limit: Uint,
    pub nonce: Bytes8,
    pub timestamp: U256,
    pub initial_balances: HashMap<Address, U256>,
}

///
///     Obtain the genesis configuration from the given genesis json file.
///
///     The genesis file should be present in the `assets` directory.
///
///     Parameters
///     ----------
///     genesis_file :
///         The json file which contains the parameters for the genesis block
///         and the pre-sale allocation data.
///
///     Returns
///     -------
///     configuration : `GenesisConfiguration`
///         The genesis configuration obtained from the json genesis file.
///
pub fn get_genesis_configuration(
    genesis_file: &str,
) -> Result<GenesisConfiguration, EthereumException> {
    let path = format!("./assets/{genesis_file}");
    let file = std::fs::read_to_string(&path).map_err(|_| EthereumException::FileNotFound(path))?;

    let value: serde_json::Value = serde_json::from_str(&file)
        .map_err(|e| EthereumException::JsonDecodeError(e.to_string()))?;

    let mut res = GenesisConfiguration::default();

    res.chain_id = U64::from(value["config"]["chainId"].as_u64().unwrap());
    res.nonce = hex_to_bytes8(value["nonce"].as_str().unwrap())?;
    res.timestamp = hex_to_u256(value["timestamp"].as_str().unwrap())?;
    res.extra_data = hex_to_bytes(value["extraData"].as_str().unwrap())?;
    res.gas_limit = hex_to_uint(value["gasLimit"].as_str().unwrap())?;
    res.difficulty = hex_to_uint(value["difficulty"].as_str().unwrap())?;

    for alloc in value["alloc"].as_object().unwrap() {
        let (address, account) = alloc;
        let address = hex_to_bytes20(address.as_str())?;
        let account = hex_to_u256(account["balance"].as_str().unwrap())?;
        res.initial_balances.insert(address, account);
    }

    Ok(res)
}

///
///     Adds the genesis block to an empty blockchain.
///
///     The genesis block is an entirely sui generis block (unique) that is not
///     governed by the general rules applying to all other Ethereum blocks.
///     Instead, the only consensus requirement is that it must be identical to
///     the block added by this function.
///
///     The mainnet genesis configuration was originally created using the
///     `mk_genesis_block.py` script. It is long since defunct, but is still
///     available at https://github.com/ethereum/genesis_block_generator.
///
///     The initial state is populated with balances based on the Ethereum presale
///     that happened on the Bitcoin blockchain. Additional Ether worth 1.98% of
///     the presale was given to the foundation.
///
///     The `state_root` is set to the root of the initial state. The `gas_limit`
///     and `difficulty` are set to suitable starting values. In particular the
///     low gas limit made sending transactions impossible in the early stages of
///     Frontier.
///
///     The `nonce` field is `0x42` referencing Douglas Adams' "HitchHiker's Guide
///     to the Galaxy".
///
///     The `extra_data` field contains the hash of block `1028201` on
///     the pre-launch Olympus testnet. The creation of block `1028201` on Olympus
///     marked the "starting gun" for Ethereum block creation. Including its hash
///     in the genesis block ensured a fair launch of the Ethereum mining process.
///
///     The remaining fields are set to appropriate default values.
///
///     On testnets the genesis configuration usually allocates 1 wei to addresses
///     `0x00` to `0xFF` to avoid edgecases around precompiles being created or
///     cleared (by EIP 161).
///
///     Parameters
///     ----------
///     hardfork:
///         The module containing the initial hardfork
///     chain :
///         An empty `Blockchain` object.
///     genesis :
///         The genesis configuration to use.
///
pub fn add_genesis_block(
    _hardfork: HardFork,
    chain: BlockChain,
    genesis: GenesisConfiguration,
) -> Result<(), EthereumException> {
    let mut state = State::default();
    for (account, balance) in genesis.initial_balances {
        state::create_ether(&mut state, account, balance);
    }

    let _genesis_header = Header {
        parent_hash: Hash32::default(),
        ommers_hash: rlp::rlp_hash(&()),
        coinbase: Address::default(),
        state_root: state_root(&chain.state),
        transactions_root: root(&Trie::<Address, _>::new(false, ()), dummy_root),
        receipt_root: root(&Trie::<Address, _>::new(false, ()), dummy_root),
        bloom: [0; 256],
        difficulty: genesis.difficulty,
        number: BigUint::default(),
        gas_limit: genesis.gas_limit,
        gas_used: BigUint::default(),
        timestamp: genesis.timestamp,
        extra_data: genesis.extra_data,
        mix_digest: Hash32::default(),
        nonce: genesis.nonce,
    };
    // genesis_block = hardfork.eth_types.Block(header = genesis_header, transactions = (), ommers = ())?;
    // chain.blocks.append(genesis_block)?;
    // chain.chain_id = genesis.chain_id;
    todo!()
}
