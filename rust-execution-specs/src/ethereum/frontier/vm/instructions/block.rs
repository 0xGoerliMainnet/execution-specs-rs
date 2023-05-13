// /// 
// /// Ethereum Virtual Machine (EVM) Block Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM block instructions.
// /// 
// use ::ethereum::base_types::{U256};
// use super::super::::{Evm};
// use super::super::gas::{GAS_BASE, GAS_BLOCK_HASH, charge_gas};
// use super::super::stack::{pop, push};
// /// 
// ///     Push the hash of one of the 256 most recent complete blocks onto the
// ///     stack. The block number to hash is present at the top of the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn block_hash(evm: Evm) -> Result<(), Error> {
//     block_number = pop(evm.stack)?;
//     charge_gas(evm, GAS_BLOCK_HASH)?;
//     if evm.env.number <= block_number || evm.env.number > block_number + 256 {
//         hash = [0];
//     } else {
//         hash = evm.env.block_hashes[-(evm.env.number - block_number)];
//     }
//     push(evm.stack, U256.from_be_bytes(hash)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the current block's beneficiary address (address of the block miner)
// ///     onto the stack.
// /// 
// ///     Here the current block refers to the block in which the currently
// ///     executing transaction/call resides.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn coinbase(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256.from_be_bytes(evm.env.coinbase)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the current block's timestamp onto the stack. Here the timestamp
// ///     being referred is actually the unix timestamp in seconds.
// /// 
// ///     Here the current block refers to the block in which the currently
// ///     executing transaction/call resides.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn timestamp(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, evm.env.time)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the current block's number onto the stack.
// /// 
// ///     Here the current block refers to the block in which the currently
// ///     executing transaction/call resides.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn number(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(evm.env.number)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the current block's difficulty onto the stack.
// /// 
// ///     Here the current block refers to the block in which the currently
// ///     executing transaction/call resides.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn difficulty(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(evm.env.difficulty)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the current block's gas limit onto the stack.
// /// 
// ///     Here the current block refers to the block in which the currently
// ///     executing transaction/call resides.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn gas_limit(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(evm.env.gas_limit)?)?;
//     evm.pc += 1;
// }


