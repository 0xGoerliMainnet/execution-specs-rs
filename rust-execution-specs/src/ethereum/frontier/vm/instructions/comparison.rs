// /// 
// /// Ethereum Virtual Machine (EVM) Comparison Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM Comparison instructions.
// /// 
// use ::ethereum::base_types::{U256};
// use super::super::::{Evm};
// use super::super::gas::{GAS_VERY_LOW, charge_gas};
// use super::super::stack::{pop, push};
// /// 
// ///     Checks if the top element is less than the next top element. Pushes the
// ///     result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn less_than(evm: Evm) -> Result<(), Error> {
//     left = pop(evm.stack)?;
//     right = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     result = U256(left < right)?;
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


// /// 
// ///     Signed less-than comparison.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn signed_less_than(evm: Evm) -> Result<(), Error> {
//     left = pop(evm.stack)?.to_signed()?;
//     right = pop(evm.stack)?.to_signed()?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     result = U256(left < right)?;
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


// /// 
// ///     Checks if the top element is greater than the next top element. Pushes
// ///     the result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn greater_than(evm: Evm) -> Result<(), Error> {
//     left = pop(evm.stack)?;
//     right = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     result = U256(left > right)?;
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


// /// 
// ///     Signed greater-than comparison.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn signed_greater_than(evm: Evm) -> Result<(), Error> {
//     left = pop(evm.stack)?.to_signed()?;
//     right = pop(evm.stack)?.to_signed()?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     result = U256(left > right)?;
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


// /// 
// ///     Checks if the top element is equal to the next top element. Pushes
// ///     the result back on the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn equal(evm: Evm) -> Result<(), Error> {
//     left = pop(evm.stack)?;
//     right = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     result = U256(left == right)?;
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


// /// 
// ///     Checks if the top element is equal to 0. Pushes the result back on the
// ///     stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn is_zero(evm: Evm) -> Result<(), Error> {
//     x = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     result = U256(x == 0)?;
//     push(evm.stack, result)?;
//     evm.pc += 1;
// }


