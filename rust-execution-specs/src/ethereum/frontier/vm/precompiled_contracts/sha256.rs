// /// 
// /// Ethereum Virtual Machine (EVM) SHA256 PRECOMPILED CONTRACT
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementation of the `SHA256` precompiled contract.
// /// 
// // NOTE: Import::Import unsupported
// use ::ethereum::base_types::{Uint};
// use ::ethereum::utils::numeric::{ceil32};
// use super::super::super::vm::{Evm};
// use super::super::super::vm::gas::{GAS_SHA256, GAS_SHA256_WORD, charge_gas};
// /// 
// ///     Writes the sha256 hash to output.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn sha256(evm: Evm) -> Result<(), Error> {
//     data = evm.message.data;
//     word_count = (ceil32(Uint(len(data)?)?)?).floordiv(32);
//     charge_gas(evm, GAS_SHA256 + GAS_SHA256_WORD * word_count)?;
//     evm.output = hashlib.sha256(data)?.digest()?;
// }


