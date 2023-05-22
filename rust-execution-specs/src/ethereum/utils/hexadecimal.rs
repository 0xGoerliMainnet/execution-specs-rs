//! 
//! # Utility Functions For Hexadecimal Strings
//! 
//! ## Introduction
//! 
//! Hexadecimal strings specific utility functions used in this specification.
//! 

// use ::ethereum::base_types::{U64, U256, Bytes, Bytes8, Bytes20, Bytes32, Bytes256, Uint};
// use ::ethereum::crypto::hash::{Hash32};

use num_traits::Num;

use crate::ethereum::{exceptions::EthereumException, base_types::{Bytes, Bytes8, Bytes20, Bytes32, Bytes256, Uint, U64, U256}, frontier::fork_types::Hash32};

/// 
///     Check if a hex string starts with hex prefix (0x).
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be checked for presence of prefix.
/// 
///     Returns
///     -------
///     has_prefix : `bool`
///         Boolean indicating whether the hex string has 0x prefix.
///     
pub fn has_hex_prefix(hex_string: &str) -> bool {
    hex_string.starts_with("0x")
}


/// 
///     Remove 0x prefix from a hex string if present. This function returns the
///     passed hex string if it isn't prefixed with 0x.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string whose prefix is to be removed.
/// 
///     Returns
///     -------
///     modified_hex_string : `str`
///         The hexadecimal string with the 0x prefix removed if present.
///     
pub fn remove_hex_prefix(hex_string: &str) -> &str {
    if has_hex_prefix(hex_string) {
        &hex_string[2..]
    } else {
        hex_string
    }
}


/// 
///     Convert hex string to bytes.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to bytes.
/// 
///     Returns
///     -------
///     byte_stream : `bytes`
///         Byte stream corresponding to the given hexadecimal string.
///     
pub fn hex_to_bytes(hex_string: &str) -> Result<Bytes, EthereumException> {
    Ok(from_hex(remove_hex_prefix(hex_string))?)
}

/// Construct a box of bytes from a hex string.
fn from_hex(hex_string: &str) -> Result<Bytes, EthereumException> {
    if hex_string.len() % 2 != 0 {
        return Err(EthereumException::BadHexString(hex_string.to_owned()));
    }
    let mut res = vec![];
    for c in hex_string.as_bytes().chunks(2) {
        // Safety: from_str_radix will fail if not uft8.
        let src = unsafe { std::str::from_utf8_unchecked(c) };
        let b = u8::from_str_radix(src, 16)
            .map_err(|_| EthereumException::BadHexString(hex_string.to_owned()))?;
        res.push(b);
    }
    Ok(Bytes::from(res))
}

fn to_bytes<const N : usize>(hex_string: &str) -> Result<[u8; N], EthereumException> {
    let hex_string = remove_hex_prefix(hex_string);
    let bytes = from_hex(hex_string)?;
    if bytes.len() > N {
        Err(EthereumException::BadHexString(hex_string.to_owned()))
    } else {
        let mut res = [0; N];
        res[N-bytes.len()..].copy_from_slice(&bytes);
        Ok(res)
    }
}

/// 
///     Convert hex string to 8 bytes.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to 8 bytes.
/// 
///     Returns
///     -------
///     8_byte_stream : `Bytes8`
///         8-byte stream corresponding to the given hexadecimal string.
///     
pub fn hex_to_bytes8(hex_string: &str) -> Result<Bytes8, EthereumException> {
    to_bytes(hex_string)
}

/// 
///     Convert hex string to 20 bytes.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to 20 bytes.
/// 
///     Returns
///     -------
///     20_byte_stream : `Bytes20`
///         20-byte stream corresponding to the given hexadecimal string.
///     
pub fn hex_to_bytes20(hex_string: &str) -> Result<Bytes20, EthereumException> {
    to_bytes(hex_string)
}


/// 
///     Convert hex string to 32 bytes.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to 32 bytes.
/// 
///     Returns
///     -------
///     32_byte_stream : `Bytes32`
///         32-byte stream corresponding to the given hexadecimal string.
///     
pub fn hex_to_bytes32(hex_string: &str) -> Result<Bytes32, EthereumException> {
    to_bytes(hex_string)
}


/// 
///     Convert hex string to 256 bytes.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to 256 bytes.
/// 
///     Returns
///     -------
///     256_byte_stream : `Bytes256`
///         256-byte stream corresponding to the given hexadecimal string.
///     
pub fn hex_to_bytes256(hex_string: &str) -> Result<Bytes256, EthereumException> {
    to_bytes(hex_string)
}


/// 
///     Convert hex string to hash32 (32 bytes).
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to hash32.
/// 
///     Returns
///     -------
///     hash : `Hash32`
///         32-byte stream obtained from the given hexadecimal string.
///     
pub fn hex_to_hash(hex_string: &str) -> Result<Hash32, EthereumException> {
    to_bytes(hex_string)
}


/// 
///     Convert hex string to Uint.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to Uint.
/// 
///     Returns
///     -------
///     converted : `Uint`
///         The unsigned integer obtained from the given hexadecimal string.
///     
pub fn hex_to_uint(hex_string: &str) -> Result<Uint, EthereumException> {
    Ok(Uint::from_str_radix(remove_hex_prefix(hex_string), 16)
        .map_err(|_| EthereumException::BadHexString(hex_string.to_owned()))?)
}


/// 
///     Convert hex string to U64.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to U256.
/// 
///     Returns
///     -------
///     converted : `U64`
///         The U64 integer obtained from the given hexadecimal string.
///     
pub fn hex_to_u64(hex_string: &str) -> Result<U64, EthereumException> {
    let bytes = hex_to_bytes8(hex_string)?;
    Ok(U64::from_be_bytes(bytes))
}


/// 
///     Convert hex string to U256.
/// 
///     Parameters
///     ----------
///     hex_string :
///         The hexadecimal string to be converted to U256.
/// 
///     Returns
///     -------
///     converted : `U256`
///         The U256 integer obtained from the given hexadecimal string.
///     
pub fn hex_to_u256(hex_string: &str) -> Result<U256, EthereumException> {
    Ok(U256::from_str_radix(remove_hex_prefix(hex_string), 16)
        .map_err(|_| EthereumException::BadHexString(hex_string.to_owned()))?)
}

/// Convert a slice of bytes to hex.
pub fn hex(bytes: &[u8]) -> String {
    let mut res = String::new();
    let hex = b"0123456789abcdef";
    res.push_str("0x");
    for b in bytes {
        res.push(hex[*b as usize >> 4].into());
        res.push(hex[*b as usize & 0x0f].into());
    }
    res
}
