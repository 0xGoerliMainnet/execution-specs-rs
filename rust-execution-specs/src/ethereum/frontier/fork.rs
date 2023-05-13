// ///
// /// Ethereum Specification
// /// ^^^^^^^^^^^^^^^^^^^^^^
// ///
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// ///
// /// Introduction
// /// ------------
// ///
// /// Entry point for the Ethereum specification.
// ///
// use ::dataclasses::{dataclass};
// use ::typing::{List, Optional, Set, Tuple};
// use ::ethereum::crypto::elliptic_curve::{SECP256K1N, secp256k1_recover};
// use ::ethereum::crypto::hash::{Hash32, keccak256};
// use ::ethereum::ethash::{dataset_size, generate_cache, hashimoto_light};
// use ::ethereum::exceptions::{InvalidBlock};
// use ::ethereum::utils::ensure::{ensure};
// use super::super::::{rlp};
// use super::super::base_types::{U64, U256, U256_CEIL_VALUE, Bytes, Bytes32, Uint};
// use super::::{vm};
// use super::bloom::{logs_bloom};
// use super::fork_types::{TX_BASE_COST, TX_DATA_COST_PER_NON_ZERO, TX_DATA_COST_PER_ZERO, Address, Block, Bloom, Header, Log, Receipt, Root, Transaction};
// use super::state::{State, create_ether, destroy_account, get_account, increment_nonce, set_account_balance, state_root};
// use super::trie::{Trie, root, trie_set};
// use super::utils::message::{prepare_message};
// use super::vm::interpreter::{process_message_call};
// BLOCK_REWARD = U256(5 * (10).pow(18))?;
// GAS_LIMIT_ADJUSTMENT_FACTOR = 1024;
// GAS_LIMIT_MINIMUM = 5000;
// MINIMUM_DIFFICULTY = Uint(131072)?;
// MAX_OMMER_DEPTH = 6;

use crate::ethereum::base_types::U64;
use super::{fork_types::Block, state::State};

///
///     History and current state of the block chain.
///     
pub struct BlockChain {
    blocks: Vec<Block>,
    state: State,
    chain_id: U64,
}

// impl BlockChain {
// }

// ///
// ///     Transforms the state from the previous hard fork (`old`) into the block
// ///     chain object for this hard fork and returns it.
// ///
// ///     When forks need to implement an irregular state transition, this function
// ///     is used to handle the irregularity. See the :ref:`DAO Fork <dao-fork>` for
// ///     an example.
// ///
// ///     Parameters
// ///     ----------
// ///     old :
// ///         Previous block chain object.
// ///
// ///     Returns
// ///     -------
// ///     new : `BlockChain`
// ///         Upgraded block chain object for this hard fork.
// ///
// pub fn apply_fork(old: BlockChain) -> Result<BlockChain, Error> {
//     return Ok(old);
// }

// ///
// ///     Obtain the list of hashes of the previous 256 blocks in order of
// ///     increasing block number.
// ///
// ///     This function will return less hashes for the first 256 blocks.
// ///
// ///     The ``BLOCKHASH`` opcode needs to access the latest hashes on the chain,
// ///     therefore this function retrieves them.
// ///
// ///     Parameters
// ///     ----------
// ///     chain :
// ///         History and current state.
// ///
// ///     Returns
// ///     -------
// ///     recent_block_hashes : `List[Hash32]`
// ///         Hashes of the recent 256 blocks in order of increasing block number.
// ///
// pub fn get_last_256_block_hashes(chain: BlockChain) -> Result<List[Hash32], Error> {
//     recent_blocks = chain.blocks[-(255)..];
//     if len(recent_blocks)? == 0 {
//         return Ok([]);
//     }
//     recent_block_hashes = [];
//     for block in recent_blocks {
//         prev_block_hash = block.header.parent_hash;
//         recent_block_hashes.append(prev_block_hash)?;
//     }
//     most_recent_block_hash = keccak256(rlp.encode(recent_blocks[-(1)].header)?)?;
//     recent_block_hashes.append(most_recent_block_hash)?;
//     return Ok(recent_block_hashes);
// }

// ///
// ///     Attempts to apply a block to an existing block chain.
// ///
// ///     All parts of the block's contents need to be verified before being added
// ///     to the chain. Blocks are verified by ensuring that the contents of the
// ///     block make logical sense with the contents of the parent block. The
// ///     information in the block's header must also match the corresponding
// ///     information in the block.
// ///
// ///     To implement Ethereum, in theory clients are only required to store the
// ///     most recent 255 blocks of the chain since as far as execution is
// ///     concerned, only those blocks are accessed. Practically, however, clients
// ///     should store more blocks to handle reorgs.
// ///
// ///     Parameters
// ///     ----------
// ///     chain :
// ///         History and current state.
// ///     block :
// ///         Block to apply to `chain`.
// ///
// pub fn state_transition(chain: BlockChain, block: Block) -> Result<(), Error> {
//     parent_header = chain.blocks[-(1)].header;
//     validate_header(block.header, parent_header)?;
//     validate_ommers(block.ommers, block.header, chain)?;
//     (gas_used, transactions_root, receipt_root, block_logs_bloom, state) = apply_body(chain.state, get_last_256_block_hashes(chain)?, block.header.coinbase, block.header.number, block.header.gas_limit, block.header.timestamp, block.header.difficulty, block.transactions, block.ommers)?;
//     ensure(gas_used == block.header.gas_used, InvalidBlock)?;
//     ensure(transactions_root == block.header.transactions_root, InvalidBlock)?;
//     ensure(state_root(state)? == block.header.state_root, InvalidBlock)?;
//     ensure(receipt_root == block.header.receipt_root, InvalidBlock)?;
//     ensure(block_logs_bloom == block.header.bloom, InvalidBlock)?;
//     chain.blocks.append(block)?;
//     if len(chain.blocks)? > 255 {
//         chain.blocks = chain.blocks[-(255)..];
//     }
// }

// ///
// ///     Verifies a block header.
// ///
// ///     In order to consider a block's header valid, the logic for the
// ///     quantities in the header should match the logic for the block itself.
// ///     For example the header timestamp should be greater than the block's parent
// ///     timestamp because the block was created *after* the parent block.
// ///     Additionally, the block's number should be directly folowing the parent
// ///     block's number since it is the next block in the sequence.
// ///
// ///     Parameters
// ///     ----------
// ///     header :
// ///         Header to check for correctness.
// ///     parent_header :
// ///         Parent Header of the header to check for correctness
// ///
// pub fn validate_header(header: Header, parent_header: Header) -> Result<(), Error> {
//     ensure(header.timestamp > parent_header.timestamp, InvalidBlock)?;
//     ensure(header.number == parent_header.number + 1, InvalidBlock)?;
//     ensure(check_gas_limit(header.gas_limit, parent_header.gas_limit)?, InvalidBlock)?;
//     ensure(len(header.extra_data)? <= 32, InvalidBlock)?;
//     block_difficulty = calculate_block_difficulty(header.number, header.timestamp, parent_header.timestamp, parent_header.difficulty)?;
//     ensure(header.difficulty == block_difficulty, InvalidBlock)?;
//     block_parent_hash = keccak256(rlp.encode(parent_header)?)?;
//     ensure(header.parent_hash == block_parent_hash, InvalidBlock)?;
//     validate_proof_of_work(header)?;
// }

// ///
// ///     Generate rlp hash of the header which is to be used for Proof-of-Work
// ///     verification.
// ///
// ///     In other words, the PoW artefacts `mix_digest` and `nonce` are ignored
// ///     while calculating this hash.
// ///
// ///     A particular PoW is valid for a single hash, that hash is computed by
// ///     this function. The `nonce` and `mix_digest` are omitted from this hash
// ///     because they are being changed by miners in their search for a sufficient
// ///     proof-of-work.
// ///
// ///     Parameters
// ///     ----------
// ///     header :
// ///         The header object for which the hash is to be generated.
// ///
// ///     Returns
// ///     -------
// ///     hash : `Hash32`
// ///         The PoW valid rlp hash of the passed in header.
// ///
// pub fn generate_header_hash_for_pow(header: Header) -> Result<Hash32, Error> {
//     header_data_without_pow_artefacts = [header.parent_hash, header.ommers_hash, header.coinbase, header.state_root, header.transactions_root, header.receipt_root, header.bloom, header.difficulty, header.number, header.gas_limit, header.gas_used, header.timestamp, header.extra_data];
//     return Ok(rlp.rlp_hash(header_data_without_pow_artefacts)?);
// }

// ///
// ///     Validates the Proof of Work constraints.
// ///
// ///     In order to verify that a miner's proof-of-work is valid for a block, a
// ///     ``mix-digest`` and ``result`` are calculated using the ``hashimoto_light``
// ///     hash function. The mix digest is a hash of the header and the nonce that
// ///     is passed through and it confirms whether or not proof-of-work was done
// ///     on the correct block. The result is the actual hash value of the block.
// ///
// ///     Parameters
// ///     ----------
// ///     header :
// ///         Header of interest.
// ///
// pub fn validate_proof_of_work(header: Header) -> Result<(), Error> {
//     header_hash = generate_header_hash_for_pow(header)?;
//     cache = generate_cache(header.number)?;
//     (mix_digest, result) = hashimoto_light(header_hash, header.nonce, cache, dataset_size(header.number)?)?;
//     ensure(mix_digest == header.mix_digest, InvalidBlock)?;
//     ensure(Uint.from_be_bytes(result)? <= (U256_CEIL_VALUE).floordiv(header.difficulty), InvalidBlock)?;
// }

// ///
// ///     Check if the transaction is includable in the block.
// ///
// ///     Parameters
// ///     ----------
// ///     tx :
// ///         The transaction.
// ///     gas_available :
// ///         The gas remaining in the block.
// ///
// ///     Returns
// ///     -------
// ///     sender_address :
// ///         The sender of the transaction.
// ///
// ///     Raises
// ///     ------
// ///     InvalidBlock :
// ///         If the transaction is not includable.
// ///
// pub fn check_transaction(tx: Transaction, gas_available: Uint) -> Result<Address, Error> {
//     ensure(tx.gas <= gas_available, InvalidBlock)?;
//     sender_address = recover_sender(tx)?;
//     return Ok(sender_address);
// }

// ///
// ///     Make the receipt for a transaction that was executed.
// ///
// ///     Parameters
// ///     ----------
// ///     tx :
// ///         The executed transaction.
// ///     post_state :
// ///         The state root immediately after this transaction.
// ///     cumulative_gas_used :
// ///         The total gas used so far in the block after the transaction was
// ///         executed.
// ///     logs :
// ///         The logs produced by the transaction.
// ///
// ///     Returns
// ///     -------
// ///     receipt :
// ///         The receipt for the transaction.
// ///
// pub fn make_receipt(tx: Transaction, post_state: Bytes32, cumulative_gas_used: Uint, logs: Tuple[Log][...]) -> Result<Receipt, Error> {
//     receipt = Receipt(post_state = post_state, cumulative_gas_used = cumulative_gas_used, bloom = logs_bloom(logs)?, logs = logs)?;
//     return Ok(receipt);
// }

// ///
// ///     Executes a block.
// ///
// ///     Many of the contents of a block are stored in data structures called
// ///     tries. There is a transactions trie which is similar to a ledger of the
// ///     transactions stored in the current block. There is also a receipts trie
// ///     which stores the results of executing a transaction, like the post state
// ///     and gas used. This function creates and executes the block that is to be
// ///     added to the chain.
// ///
// ///     Parameters
// ///     ----------
// ///     state :
// ///         Current account state.
// ///     block_hashes :
// ///         List of hashes of the previous 256 blocks in the order of
// ///         increasing block number.
// ///     coinbase :
// ///         Address of account which receives block reward and transaction fees.
// ///     block_number :
// ///         Position of the block within the chain.
// ///     block_gas_limit :
// ///         Initial amount of gas available for execution in this block.
// ///     block_time :
// ///         Time the block was produced, measured in seconds since the epoch.
// ///     block_difficulty :
// ///         Difficulty of the block.
// ///     transactions :
// ///         Transactions included in the block.
// ///     ommers :
// ///         Headers of ancestor blocks which are not direct parents (formerly
// ///         uncles.)
// ///
// ///     Returns
// ///     -------
// ///     gas_available : `ethereum.base_types.Uint`
// ///         Remaining gas after all transactions have been executed.
// ///     transactions_root : `ethereum.fork_types.Root`
// ///         Trie root of all the transactions in the block.
// ///     receipt_root : `ethereum.fork_types.Root`
// ///         Trie root of all the receipts in the block.
// ///     block_logs_bloom : `Bloom`
// ///         Logs bloom of all the logs included in all the transactions of the
// ///         block.
// ///     state : `ethereum.fork_types.State`
// ///         State after all transactions have been executed.
// ///
// pub fn apply_body(state: State, block_hashes: List[Hash32], coinbase: Address, block_number: Uint, block_gas_limit: Uint, block_time: U256, block_difficulty: Uint, transactions: Tuple[Transaction][...], ommers: Tuple[Header][...]) -> Result<Tuple[Uint][Root][Root][Bloom][State], Error> {
//     gas_available = block_gas_limit;
//     // TypedAssignment unsupported
//     // TypedAssignment unsupported
//     // TypedAssignment unsupported
//     for (i, tx) in enumerate(transactions)? {
//         trie_set(transactions_trie, rlp.encode(Uint(i)?)?, tx)?;
//         sender_address = check_transaction(tx, gas_available)?;
//         env = vm.Environment(caller = sender_address, origin = sender_address, block_hashes = block_hashes, coinbase = coinbase, number = block_number, gas_limit = block_gas_limit, gas_price = tx.gas_price, time = block_time, difficulty = block_difficulty, state = state)?;
//         (gas_used, logs) = process_transaction(env, tx)?;
//         gas_available -= gas_used;
//         receipt = make_receipt(tx, state_root(state)?, block_gas_limit - gas_available, logs)?;
//         trie_set(receipts_trie, rlp.encode(Uint(i)?)?, receipt)?;
//         block_logs += logs;
//     }
//     pay_rewards(state, block_number, coinbase, ommers)?;
//     block_gas_used = block_gas_limit - gas_available;
//     block_logs_bloom = logs_bloom(block_logs)?;
//     return Ok((block_gas_used, root(transactions_trie)?, root(receipts_trie)?, block_logs_bloom, state));
// }

// ///
// ///     Validates the ommers mentioned in the block.
// ///
// ///     An ommer block is a block that wasn't canonically added to the
// ///     blockchain because it wasn't validated as fast as the canonical block
// ///     but was mined at the same time.
// ///
// ///     To be considered valid, the ommers must adhere to the rules defined in
// ///     the Ethereum protocol. The maximum amount of ommers is 2 per block and
// ///     there cannot be duplicate ommers in a block. Many of the other ommer
// ///     contraints are listed in the in-line comments of this function.
// ///
// ///     Parameters
// ///     ----------
// ///     ommers :
// ///         List of ommers mentioned in the current block.
// ///     block_header:
// ///         The header of current block.
// ///     chain :
// ///         History and current state.
// ///
// pub fn validate_ommers(ommers: Tuple[Header][...], block_header: Header, chain: BlockChain) -> Result<(), Error> {
//     block_hash = rlp.rlp_hash(block_header)?;
//     ensure(rlp.rlp_hash(ommers)? == block_header.ommers_hash, InvalidBlock)?;
//     if len(ommers)? == 0 {
//         return;
//     }
//     for ommer in ommers {
//         ensure(1 <= ommer.number < block_header.number, InvalidBlock)?;
//         ommer_parent_header = chain.blocks[-(block_header.number - ommer.number) - 1].header;
//         validate_header(ommer, ommer_parent_header)?;
//     }
//     ensure(len(ommers)? <= 2, InvalidBlock)?;
//     ommers_hashes = /* ListComp unsupported */;
//     ensure(len(ommers_hashes)? == len(set(ommers_hashes)?)?, InvalidBlock)?;
//     recent_canonical_blocks = chain.blocks[-(MAX_OMMER_DEPTH + 1)..];
//     recent_canonical_block_hashes = /* SetComp unsupported */;
//     // TypedAssignment unsupported
//     for block in recent_canonical_blocks {
//         recent_ommers_hashes = recent_ommers_hashes.union(/* SetComp unsupported */)?;
//     }
//     for (ommer_index, ommer) in enumerate(ommers)? {
//         ommer_hash = ommers_hashes[ommer_index];
//         ensure(ommer_hash != block_hash, InvalidBlock)?;
//         ensure(!(ommer_hash).contains(recent_canonical_block_hashes), InvalidBlock)?;
//         ensure(!(ommer_hash).contains(recent_ommers_hashes), InvalidBlock)?;
//         ommer_age = block_header.number - ommer.number;
//         ensure(1 <= ommer_age <= MAX_OMMER_DEPTH, InvalidBlock)?;
//         ensure((ommer.parent_hash).contains(recent_canonical_block_hashes), InvalidBlock)?;
//         ensure(ommer.parent_hash != block_header.parent_hash, InvalidBlock)?;
//     }
// }

// ///
// ///     Pay rewards to the block miner as well as the ommers miners.
// ///
// ///     The miner of the canonical block is rewarded with the predetermined
// ///     block reward, ``BLOCK_REWARD``, plus a variable award based off of the
// ///     number of ommer blocks that were mined around the same time, and included
// ///     in the canonical block's header. An ommer block is a block that wasn't
// ///     added to the canonical blockchain because it wasn't validated as fast as
// ///     the accepted block but was mined at the same time. Although not all blocks
// ///     that are mined are added to the canonical chain, miners are still paid a
// ///     reward for their efforts. This reward is called an ommer reward and is
// ///     calculated based on the number associated with the ommer block that they
// ///     mined.
// ///
// ///     Parameters
// ///     ----------
// ///     state :
// ///         Current account state.
// ///     block_number :
// ///         Position of the block within the chain.
// ///     coinbase :
// ///         Address of account which receives block reward and transaction fees.
// ///     ommers :
// ///         List of ommers mentioned in the current block.
// ///
// pub fn pay_rewards(state: State, block_number: Uint, coinbase: Address, ommers: Tuple[Header][...]) -> Result<(), Error> {
//     miner_reward = BLOCK_REWARD + len(ommers)? * (BLOCK_REWARD).floordiv(32);
//     create_ether(state, coinbase, miner_reward)?;
//     for ommer in ommers {
//         ommer_age = U256(block_number - ommer.number)?;
//         ommer_miner_reward = (8 - ommer_age * BLOCK_REWARD).floordiv(8);
//         create_ether(state, ommer.coinbase, ommer_miner_reward)?;
//     }
// }

// ///
// ///     Execute a transaction against the provided environment.
// ///
// ///     This function processes the actions needed to execute a transaction.
// ///     It decrements the sender's account after calculating the gas fee and
// ///     refunds them the proper amount after execution. Calling contracts,
// ///     deploying code, and incrementing nonces are all examples of actions that
// ///     happen within this function or from a call made within this function.
// ///
// ///     Accounts that are marked for deletion are processed and destroyed after
// ///     execution.
// ///
// ///     Parameters
// ///     ----------
// ///     env :
// ///         Environment for the Ethereum Virtual Machine.
// ///     tx :
// ///         Transaction to execute.
// ///
// ///     Returns
// ///     -------
// ///     gas_left : `ethereum.base_types.U256`
// ///         Remaining gas after execution.
// ///     logs : `Tuple[ethereum.fork_types.Log, ...]`
// ///         Logs generated during execution.
// ///
// pub fn process_transaction(env: vm.Environment, tx: Transaction) -> Result<Tuple[U256][Tuple[Log][...]], Error> {
//     ensure(validate_transaction(tx)?, InvalidBlock)?;
//     sender = env.origin;
//     sender_account = get_account(env.state, sender)?;
//     gas_fee = tx.gas * tx.gas_price;
//     ensure(sender_account.nonce == tx.nonce, InvalidBlock)?;
//     ensure(sender_account.balance >= gas_fee + tx.value, InvalidBlock)?;
//     ensure(sender_account.code == bytearray()?, InvalidBlock)?;
//     gas = tx.gas - calculate_intrinsic_cost(tx)?;
//     increment_nonce(env.state, sender)?;
//     sender_balance_after_gas_fee = sender_account.balance - gas_fee;
//     set_account_balance(env.state, sender, sender_balance_after_gas_fee)?;
//     message = prepare_message(sender, tx.to, tx.value, tx.data, gas, env)?;
//     output = process_message_call(message, env)?;
//     gas_used = tx.gas - output.gas_left;
//     gas_refund = min((gas_used).floordiv(2), output.refund_counter)?;
//     gas_refund_amount = output.gas_left + gas_refund * tx.gas_price;
//     transaction_fee = tx.gas - output.gas_left - gas_refund * tx.gas_price;
//     total_gas_used = gas_used - gas_refund;
//     sender_balance_after_refund = get_account(env.state, sender)?.balance + gas_refund_amount;
//     set_account_balance(env.state, sender, sender_balance_after_refund)?;
//     coinbase_balance_after_mining_fee = get_account(env.state, env.coinbase)?.balance + transaction_fee;
//     set_account_balance(env.state, env.coinbase, coinbase_balance_after_mining_fee)?;
//     for address in output.accounts_to_delete {
//         destroy_account(env.state, address)?;
//     }
//     return Ok((total_gas_used, output.logs));
// }

// ///
// ///     Verifies a transaction.
// ///
// ///     The gas in a transaction gets used to pay for the intrinsic cost of
// ///     operations, therefore if there is insufficient gas then it would not
// ///     be possible to execute a transaction and it will be declared invalid.
// ///
// ///     Additionally, the nonce of a transaction must not equal or exceed the
// ///     limit defined in `EIP-2681 <https://eips.ethereum.org/EIPS/eip-2681>`_.
// ///     In practice, defining the limit as ``2**64-1`` has no impact because
// ///     sending ``2**64-1`` transactions is improbable. It's not strictly
// ///     impossible though, ``2**64-1`` transactions is the entire capacity of the
// ///     Ethereum blockchain at 2022 gas limits for a little over 22 years.
// ///
// ///     Parameters
// ///     ----------
// ///     tx :
// ///         Transaction to validate.
// ///
// ///     Returns
// ///     -------
// ///     verified : `bool`
// ///         True if the transaction can be executed, or False otherwise.
// ///
// pub fn validate_transaction(tx: Transaction) -> Result<bool, Error> {
//     return Ok(calculate_intrinsic_cost(tx)? <= tx.gas && tx.nonce < (2).pow(64) - 1);
// }

// ///
// ///     Calculates the gas that is charged before execution is started.
// ///
// ///     The intrinsic cost of the transaction is charged before execution has
// ///     begun. Functions/operations in the EVM cost money to execute so this
// ///     intrinsic cost is for the operations that need to be paid for as part of
// ///     the transaction. Data transfer, for example, is part of this intrinsic
// ///     cost. It costs ether to send data over the wire and that ether is
// ///     accounted for in the intrinsic cost calculated in this function. This
// ///     intrinsic cost must be calculated and paid for before execution in order
// ///     for all operations to be implemented.
// ///
// ///     Parameters
// ///     ----------
// ///     tx :
// ///         Transaction to compute the intrinsic cost of.
// ///
// ///     Returns
// ///     -------
// ///     verified : `ethereum.base_types.Uint`
// ///         The intrinsic cost of the transaction.
// ///
// pub fn calculate_intrinsic_cost(tx: Transaction) -> Result<Uint, Error> {
//     data_cost = 0;
//     for byte in tx.data {
//         if byte == 0 {
//             data_cost += TX_DATA_COST_PER_ZERO;
//         } else {
//             data_cost += TX_DATA_COST_PER_NON_ZERO;
//         }
//     }
//     return Ok(Uint(TX_BASE_COST + data_cost)?);
// }

// ///
// ///     Extracts the sender address from a transaction.
// ///
// ///     The v, r, and s values are the three parts that make up the signature
// ///     of a transaction. In order to recover the sender of a transaction the two
// ///     components needed are the signature (``v``, ``r``, and ``s``) and the
// ///     signing hash of the transaction. The sender's public key can be obtained
// ///     with these two values and therefore the sender address can be retrieved.
// ///
// ///     Parameters
// ///     ----------
// ///     tx :
// ///         Transaction of interest.
// ///
// ///     Returns
// ///     -------
// ///     sender : `ethereum.fork_types.Address`
// ///         The address of the account that signed the transaction.
// ///
// pub fn recover_sender(tx: Transaction) -> Result<Address, Error> {
//     (v, r, s) = tx.v = tx.r = tx.s;
//     ensure(v == 27 || v == 28, InvalidBlock)?;
//     ensure(0 < r && r < SECP256K1N, InvalidBlock)?;
//     ensure(0 < s && s < SECP256K1N, InvalidBlock)?;
//     public_key = secp256k1_recover(r, s, v - 27, signing_hash(tx)?)?;
//     return Ok(Address(keccak256(public_key)?[12..32])?);
// }

// ///
// ///     Compute the hash of a transaction used in the signature.
// ///
// ///     The values that are used to compute the signing hash set the rules for a
// ///     transaction. For example, signing over the gas sets a limit for the
// ///     amount of money that is allowed to be pulled out of the sender's account.
// ///
// ///     Parameters
// ///     ----------
// ///     tx :
// ///         Transaction of interest.
// ///
// ///     Returns
// ///     -------
// ///     hash : `ethereum.crypto.hash.Hash32`
// ///         Hash of the transaction.
// ///
// pub fn signing_hash(tx: Transaction) -> Result<Hash32, Error> {
//     return Ok(keccak256(rlp.encode((tx.nonce, tx.gas_price, tx.gas, tx.to, tx.value, tx.data))?)?);
// }

// ///
// ///     Computes the hash of a block header.
// ///
// ///     The header hash of a block is the canonical hash that is used to refer
// ///     to a specific block and completely distinguishes a block from another.
// ///
// ///     ``keccak256`` is a function that produces a 256 bit hash of any input.
// ///     It also takes in any number of bytes as an input and produces a single
// ///     hash for them. A hash is a completely unique output for a single input.
// ///     So an input corresponds to one unique hash that can be used to identify
// ///     the input exactly.
// ///
// ///     Prior to using the ``keccak256`` hash function, the header must be
// ///     encoded using the Recursive-Length Prefix. See :ref:`rlp`.
// ///     RLP encoding the header converts it into a space-efficient format that
// ///     allows for easy transfer of data between nodes. The purpose of RLP is to
// ///     encode arbitrarily nested arrays of binary data, and RLP is the primary
// ///     encoding method used to serialize objects in Ethereum's execution layer.
// ///     The only purpose of RLP is to encode structure; encoding specific data
// ///     types (e.g. strings, floats) is left up to higher-order protocols.
// ///
// ///     Parameters
// ///     ----------
// ///     header :
// ///         Header of interest.
// ///
// ///     Returns
// ///     -------
// ///     hash : `ethereum.crypto.hash.Hash32`
// ///         Hash of the header.
// ///
// pub fn compute_header_hash(header: Header) -> Result<Hash32, Error> {
//     return Ok(keccak256(rlp.encode(header)?)?);
// }

// ///
// ///     Validates the gas limit for a block.
// ///
// ///     The bounds of the gas limit, ``max_adjustment_delta``, is set as the
// ///     quotient of the parent block's gas limit and the
// ///     ``GAS_LIMIT_ADJUSTMENT_FACTOR``. Therefore, if the gas limit that is
// ///     passed through as a parameter is greater than or equal to the *sum* of
// ///     the parent's gas and the adjustment delta then the limit for gas is too
// ///     high and fails this function's check. Similarly, if the limit is less
// ///     than or equal to the *difference* of the parent's gas and the adjustment
// ///     delta *or* the predefined ``GAS_LIMIT_MINIMUM`` then this function's
// ///     check fails because the gas limit doesn't allow for a sufficient or
// ///     reasonable amount of gas to be used on a block.
// ///
// ///     Parameters
// ///     ----------
// ///     gas_limit :
// ///         Gas limit to validate.
// ///
// ///     parent_gas_limit :
// ///         Gas limit of the parent block.
// ///
// ///     Returns
// ///     -------
// ///     check : `bool`
// ///         True if gas limit constraints are satisfied, False otherwise.
// ///
// pub fn check_gas_limit(gas_limit: Uint, parent_gas_limit: Uint) -> Result<bool, Error> {
//     max_adjustment_delta = (parent_gas_limit).floordiv(GAS_LIMIT_ADJUSTMENT_FACTOR);
//     if gas_limit >= parent_gas_limit + max_adjustment_delta {
//         return Ok(false);
//     }
//     if gas_limit <= parent_gas_limit - max_adjustment_delta {
//         return Ok(false);
//     }
//     if gas_limit < GAS_LIMIT_MINIMUM {
//         return Ok(false);
//     }
//     return Ok(true);
// }

// ///
// ///     Computes difficulty of a block using its header and
// ///     parent header.
// ///
// ///     The difficulty of a block is determined by the time the block was
// ///     created after its parent. If a block's timestamp is more than 13
// ///     seconds after its parent block then its difficulty is set as the
// ///     difference between the parent's difficulty and the
// ///     ``max_adjustment_delta``. Otherwise, if the time between parent and
// ///     child blocks is too small (under 13 seconds) then, to avoid mass
// ///     forking, the block's difficulty is set to the sum of the delta and
// ///     the parent's difficulty.
// ///
// ///     Parameters
// ///     ----------
// ///     block_number :
// ///         Block number of the block.
// ///     block_timestamp :
// ///         Timestamp of the block.
// ///     parent_timestamp :
// ///         Timestamp of the parent block.
// ///     parent_difficulty :
// ///         difficulty of the parent block.
// ///
// ///     Returns
// ///     -------
// ///     difficulty : `ethereum.base_types.Uint`
// ///         Computed difficulty for a block.
// ///
// pub fn calculate_block_difficulty(block_number: Uint, block_timestamp: U256, parent_timestamp: U256, parent_difficulty: Uint) -> Result<Uint, Error> {
//     max_adjustment_delta = (parent_difficulty).floordiv(Uint(2048)?);
//     if block_timestamp < parent_timestamp + 13 {
//         difficulty = parent_difficulty + max_adjustment_delta;
//     } else {
//         difficulty = parent_difficulty - max_adjustment_delta;
//     }
//     num_bomb_periods = (int(block_number)?).floordiv(100000) - 2;
//     if num_bomb_periods >= 0 {
//         difficulty += (2).pow(num_bomb_periods);
//     }
//     return Ok(max(difficulty, MINIMUM_DIFFICULTY)?);
// }
