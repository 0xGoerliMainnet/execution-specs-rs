// /// 
// /// Ethereum Virtual Machine (EVM) ECRECOVER PRECOMPILED CONTRACT
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementation of the ECRECOVER precompiled contract.
// /// 
// use ::ethereum::base_types::{U256};
// use ::ethereum::crypto::elliptic_curve::{SECP256K1N, secp256k1_recover};
// use ::ethereum::crypto::hash::{Hash32, keccak256};
// use ::ethereum::utils::byte::{left_pad_zero_bytes};
// use super::super::super::vm::{Evm};
// use super::super::super::vm::gas::{GAS_ECRECOVER, charge_gas};
// use super::super::super::vm::memory::{buffer_read};
// /// 
// ///     Decrypts the address using elliptic curve DSA recovery mechanism and writes
// ///     the address to output.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn ecrecover(evm: Evm) -> Result<(), Error> {
//     data = evm.message.data;
//     charge_gas(evm, GAS_ECRECOVER)?;
//     message_hash_bytes = buffer_read(data, U256(0)?, U256(32)?)?;
//     message_hash = Hash32(message_hash_bytes)?;
//     v = U256.from_be_bytes(buffer_read(data, U256(32)?, U256(32)?)?)?;
//     r = U256.from_be_bytes(buffer_read(data, U256(64)?, U256(32)?)?)?;
//     s = U256.from_be_bytes(buffer_read(data, U256(96)?, U256(32)?)?)?;
//     if v != 27 && v != 28 {
//         return;
//     }
//     if 0 >= r || r >= SECP256K1N {
//         return;
//     }
//     if 0 >= s || s >= SECP256K1N {
//         return;
//     }
//     // Try 
//         public_key = secp256k1_recover(r, s, v - 27, message_hash)?;
//     address = keccak256(public_key)?[12..32];
//     padded_address = left_pad_zero_bytes(address, 32)?;
//     evm.output = padded_address;
// }


