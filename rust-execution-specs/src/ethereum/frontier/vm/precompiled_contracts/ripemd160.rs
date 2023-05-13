// /// 
// /// Ethereum Virtual Machine (EVM) RIPEMD160 PRECOMPILED CONTRACT
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementation of the `RIPEMD160` precompiled contract.
// /// 
// // NOTE: Import::Import unsupported
// use ::ethereum::base_types::{Uint};
// use ::ethereum::utils::byte::{left_pad_zero_bytes};
// use ::ethereum::utils::numeric::{ceil32};
// use super::super::super::vm::{Evm};
// use super::super::super::vm::gas::{GAS_RIPEMD160, GAS_RIPEMD160_WORD, charge_gas};
// /// 
// ///     Writes the ripemd160 hash to output.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn ripemd160(evm: Evm) -> Result<(), Error> {
//     data = evm.message.data;
//     word_count = (ceil32(Uint(len(data)?)?)?).floordiv(32);
//     charge_gas(evm, GAS_RIPEMD160 + GAS_RIPEMD160_WORD * word_count)?;
//     hash_bytes = hashlib.new("ripemd160", data)?.digest()?;
//     padded_hash = left_pad_zero_bytes(hash_bytes, 32)?;
//     evm.output = padded_hash;
// }


