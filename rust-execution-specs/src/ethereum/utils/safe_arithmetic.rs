//! 
//! Safe Arithmetic for U256 Integer Type
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! 
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//! 
//! Introduction
//! ------------
//! 
//! Safe arithmetic utility functions for U256 integer type.
//! 

// use ::typing::{Optional, Type, Union};
// use ::ethereum::base_types::{U256, Uint};
// /// 
// ///     Adds together the given sequence of numbers. If the total sum of the
// ///     numbers exceeds `U256.MAX_VALUE` then an exception is raised.
// ///     If `exception_type` = None then the exception raised defaults to the one
// ///     raised by `U256` when `U256.value > U256.MAX_VALUE`
// ///     else `exception_type` is raised.
// /// 
// ///     Parameters
// ///     ----------
// ///     numbers :
// ///         The sequence of numbers that need to be added together.
// /// 
// ///     exception_type:
// ///         The exception that needs to be raised if the sum of the `numbers`
// ///         exceeds `U256.MAX_VALUE`.
// /// 
// ///     Returns
// ///     -------
// ///     result : `ethereum.base_types.U256`
// ///         The sum of the given sequence of numbers if the total is less than
// ///         `U256.MAX_VALUE` else an exception is raised.
// ///         If `exception_type` = None then the exception raised defaults to the
// ///         one raised by `U256` when `U256.value > U256.MAX_VALUE`
// ///         else `exception_type` is raised.
// ///     
// pub fn u256_safe_add() -> Result<U256, Error> {
//     // Try 
//         return Ok(U256(sum(numbers)?)?);
// }


// /// 
// ///     Multiplies together the given sequence of numbers. If the net product of
// ///     the numbers exceeds `U256.MAX_VALUE` then an exception is raised.
// ///     If `exception_type` = None then the exception raised defaults to the one
// ///     raised by `U256` when `U256.value > U256.MAX_VALUE` else
// ///     `exception_type` is raised.
// /// 
// ///     Parameters
// ///     ----------
// ///     numbers :
// ///         The sequence of numbers that need to be multiplies together.
// /// 
// ///     exception_type:
// ///         The exception that needs to be raised if the sum of the `numbers`
// ///         exceeds `U256.MAX_VALUE`.
// /// 
// ///     Returns
// ///     -------
// ///     result : `ethereum.base_types.U256`
// ///         The multiplication product of the given sequence of numbers if the
// ///         net product  is less than `U256.MAX_VALUE` else an exception is raised.
// ///         If `exception_type` = None then the exception raised defaults to the
// ///         one raised by `U256` when `U256.value > U256.MAX_VALUE`
// ///         else `exception_type` is raised.
// ///     
// pub fn u256_safe_multiply() -> Result<U256, Error> {
//     result = numbers[0];
//     // Try 
//         for number in numbers[1..] {
//             result *= number;
//         }
//         return Ok(U256(result)?);
// }


