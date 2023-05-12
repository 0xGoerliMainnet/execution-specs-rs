//! 
//! Utility Functions For Byte Strings
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! 
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//! 
//! Introduction
//! ------------
//! 
//! Byte specific utility functions used in this specification.
//!
 
// use ::ethereum::base_types::{Bytes};
// /// 
// ///     Left pad zeroes to `value` if it's length is less than the given `size`.
// /// 
// ///     Parameters
// ///     ----------
// ///     value :
// ///         The byte string that needs to be padded.
// ///     size :
// ///         The number of bytes that need that need to be padded.
// /// 
// ///     Returns
// ///     -------
// ///     left_padded_value: `ethereum.base_types.Bytes`
// ///         left padded byte string of given `size`.
// ///     
// pub fn left_pad_zero_bytes(value: Bytes, size: int) -> Result<Bytes, Error> {
//     return Ok(value.rjust(size, [0])?);
// }


// /// 
// ///     Right pad zeroes to `value` if it's length is less than the given `size`.
// /// 
// ///     Parameters
// ///     ----------
// ///     value :
// ///         The byte string that needs to be padded.
// ///     size :
// ///         The number of bytes that need that need to be padded.
// /// 
// ///     Returns
// ///     -------
// ///     right_padded_value: `ethereum.base_types.Bytes`
// ///         right padded byte string of given `size`.
// ///     
// pub fn right_pad_zero_bytes(value: Bytes, size: int) -> Result<Bytes, Error> {
//     return Ok(value.ljust(size, [0])?);
// }


