//!
//! # Numeric & Array Types
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Integer and array types which are used by—but not unique to—Ethereum.
//!


// use ::__future__::{annotations};
// use ::dataclasses::{replace};
// use ::typing::{Any, Callable, ClassVar, Optional, Tuple, Type, TypeVar};
// U8_MAX_VALUE = (2).pow(8) - 1;
// U32_MAX_VALUE = (2).pow(32) - 1;
// U32_CEIL_VALUE = (2).pow(32);
// U64_MAX_VALUE = (2).pow(64) - 1;
// U255_MAX_VALUE = (2).pow(255) - 1;
// U255_CEIL_VALUE = (2).pow(255);
// U256_MAX_VALUE = (2).pow(256) - 1;
// U256_CEIL_VALUE = (2).pow(256);

use num_bigint::BigUint;

// use super::exceptions::EthereumException;

/// Array of bytes.
pub type Bytes = Box<[u8]>;

// pub type SignedInt = num_bigint::BigInt;


// /// 
// ///     Unsigned positive integer.
// ///     
pub type Uint = num_bigint::BigUint;

///
///     Unsigned positive integer, which can represent `0` to `2 ** 256 - 1`,
///     inclusive.
///
///
pub type U256 = BigUint;

/// 
///     Unsigned positive integer, which can represent `0` to `2 ** 32 - 1`,
///     inclusive.
///     
pub type U32 = u32;

/// 
///     Unsigned positive integer, which can represent `0` to `2 ** 64 - 1`,
///     inclusive.
///     
pub type U64 = u64;



///
///     Byte array of exactly zero elements.
///
pub type Bytes0 = [u8; 0];

///
///     Byte array of exactly four elements.
///
pub type Bytes4 = [u8; 4];

///
///     Byte array of exactly eight elements.
///
pub type Bytes8 = [u8; 8];


///
///     Byte array of exactly 20 elements.
///
pub type Bytes20 = [u8; 20];

///
///     Byte array of exactly 32 elements.
///
pub type Bytes32 = [u8; 32];

///
///     Byte array of exactly 64 elements.
///
pub type Bytes64 = [u8; 64];

///
///     Byte array of exactly 256 elements.
///
pub type Bytes256 = [u8; 256];

/// Does exactly what it says on the tin.
pub fn strip_leading_zeros(value: &[u8]) -> &[u8] {
    let leading_zeros = value.iter().position(|b| *b != 0).unwrap_or(value.len());
    &value[leading_zeros..]
}
