// /// 
// /// Ethereum Virtual Machine (EVM) IDENTITY PRECOMPILED CONTRACT
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementation of the `IDENTITY` precompiled contract.
// /// 
// use ::ethereum::base_types::{Uint};
// use ::ethereum::utils::numeric::{ceil32};
// use super::super::super::vm::{Evm};
// use super::super::super::vm::gas::{GAS_IDENTITY, GAS_IDENTITY_WORD, charge_gas};
// /// 
// ///     Writes the message data to output.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn identity(evm: Evm) -> Result<(), Error> {
//     data = evm.message.data;
//     word_count = (ceil32(Uint(len(data)?)?)?).floordiv(32);
//     charge_gas(evm, GAS_IDENTITY + GAS_IDENTITY_WORD * word_count)?;
//     evm.output = data;
// }


