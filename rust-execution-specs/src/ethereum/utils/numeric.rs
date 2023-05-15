//! 
//! # Utility Functions For Numeric Operations
//! 
//! ## Introduction
//! 
//! Numeric operations specific utility functions used in this specification.
//!

use num_bigint::BigInt;
use crate::ethereum::{base_types::{Uint, Bytes}, exceptions::EthereumException};


/// Determines the sign of a number.
/// 
/// Parameters
/// ----------
/// value :
///     The value whose sign is to be determined.
/// 
/// Returns
/// -------
/// sign : `int`
///     The sign of the number (-1 or 0 or 1).
///     The return value is based on math signum function.
pub fn get_sign(value: BigInt) -> BigInt {
    if value < BigInt::from(0) {
        BigInt::from(-1)
    } else if value == BigInt::from(0) {
        BigInt::from(0)
    } else {
        BigInt::from(1)
    }
}



///
///     Converts a unsigned integer to the next closest multiple of 32.
///
///     Parameters
///     ----------
///     value :
///         The value whose ceil32 is to be calculated.
///
///     Returns
///     -------
///     ceil32 : `ethereum.base_types.U256`
///         The same value if it's a perfect multiple of 32
///         else it returns the smallest multiple of 32
///         that is greater than `value`.
///
pub fn ceil32(value: Uint) -> Uint {
    let ceiling = Uint::from(32u8);
    let remainder = &value % &ceiling;
    if remainder == Uint::from(0u8) {
        value
    } else {
        value + ceiling - remainder
    }
}

// /// 
// ///     Checks if `number` is a prime number.
// /// 
// ///     Parameters
// ///     ----------
// ///     number :
// ///         The number to check for primality.
// /// 
// ///     Returns
// ///     -------
// ///     is_number_prime : `bool`
// ///         Boolean indicating if `number` is prime or not.
// ///     
// pub fn is_prime(number: int) -> Result<bool, Error> {
//     if number <= 1 {
//         return Ok(false);
//     }
//     for x in range(2, int((number).pow(0.5))? + 1)? {
//         if number % x == 0 {
//             return Ok(false);
//         }
//     }
//     return Ok(true);
// }


/// 
///     Convert little endian byte stream `data` to a little endian U32
///     sequence i.e., the first U32 number of the sequence is the least
///     significant U32 number.
/// 
///     Parameters
///     ----------
///     data :
///         The byte stream (little endian) which is to be converted to a U32
///         stream.
/// 
///     Returns
///     -------
///     uint32_sequence : `Tuple[U32, ...]`
///         Sequence of U32 numbers obtained from the little endian byte
///         stream.
///     
pub fn le_bytes_to_uint32_sequence(data: &[u8]) -> Vec<u32> {
    let mut sequence = Vec::new();

    for i in (0..data.len()).step_by(4) {
        let value = u32::from_le_bytes([
            data[i],
            data[i + 1],
            data[i + 2],
            data[i + 3],
        ]);
        sequence.push(value);
    }

    sequence
}

/// 
///     Obtain little endian byte stream from a little endian U32 sequence
///     i.e., the first U32 number of the sequence is the least significant
///     U32 number.
/// 
///     Note - In this conversion, the most significant byte (byte at the end of
///     the little endian stream) may have leading zeroes. This function doesn't
///     take care of removing these leading zeroes as shown in below example.
/// 
///     >>> le_uint32_sequence_to_bytes([U32(8)])
///     b'\x08\x00\x00\x00'
/// 
/// 
///     Parameters
///     ----------
///     sequence :
///         The U32 stream (little endian) which is to be converted to a
///         little endian byte stream.
/// 
///     Returns
///     -------
///     result : `bytes`
///         The byte stream obtained from the little endian U32 stream.
///     
pub fn le_uint32_sequence_to_bytes(sequence: &Vec<u32>) -> Result<Bytes, EthereumException> {
    let mut result_bytes = Vec::new();
    for item in sequence {
        result_bytes.extend(item.to_le_bytes());
    }

    return Ok(Bytes::from_iter(result_bytes));
}


/// 
///     Obtain Uint from a U32 sequence assuming that this sequence is little
///     endian i.e., the first U32 number of the sequence is the least
///     significant U32 number.
/// 
///     Parameters
///     ----------
///     sequence :
///         The U32 stream (little endian) which is to be converted to a Uint.
/// 
///     Returns
///     -------
///     value : `Uint`
///         The Uint number obtained from the conversion of the little endian
///         U32 stream.
///     
pub fn le_uint32_sequence_to_uint(sequence: &Vec<u32>) -> Result<Uint, EthereumException> {
    let sequence_as_bytes = le_uint32_sequence_to_bytes(sequence)?;
    return Ok(Uint::from_bytes_le(Box::leak(sequence_as_bytes)));
}
