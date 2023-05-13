use num_bigint::BigInt;
use num_traits::Signed;
use crate::ethereum::base_types::U256;
use crate::ethereum::utils::numeric::get_sign;
use super::{
    super::{Evm, exceptions::{Result, EvmError}, stack, gas}
};

/// Adds the top two elements of the stack together, and pushes the result back
/// on the stack.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn add(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;
    
    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;
    
    // OPERATION
    // fixme: wrapping_add
    let result = x + y;
    
    stack::push(&mut evm.stack, result)?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Subtracts the top two elements of the stack, and pushes the result back
/// on the stack.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn sub(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;
    
    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;
    
    // OPERATION
    // fixme: wrapping_sub
    let result = x - y;
    
    stack::push(&mut evm.stack, result)?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Multiply the top two elements of the stack, and pushes the result back
/// on the stack.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn mul(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;
    
    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;
    
    // OPERATION
    // fixme: wrapping_mul
    let result = x * y;
    
    stack::push(&mut evm.stack, result)?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Integer division of the top two elements of the stack. Pushes the result
/// back on the stack.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn div(evm: &mut Evm) -> Result<()> {
    // STACK
    let dividend = stack::pop(&mut evm.stack)?;
    let divisor = stack::pop(&mut evm.stack)?;
    
    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;
    
    // OPERATION
    let quotient = if divisor == U256::from(0u8) {
        U256::from(0u8)
    } else {
        dividend / divisor
    };
    
    stack::push(&mut evm.stack, quotient)?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Signed integer division of the top two elements of the stack. Pushes the
/// result back on the stack.
/// 
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn sdiv(evm: &mut Evm) -> Result<()> {
    // STACK
    let dividend = BigInt::from(stack::pop(&mut evm.stack)?);
    let divisor = BigInt::from(stack::pop(&mut evm.stack)?);
    
    // GAS
    gas::charge_gas(evm, gas::GAS_VERY_LOW())?;
    
    // OPERATION
    let quotient = if divisor == BigInt::from(0u8) {
        BigInt::from(0u8)
    } else if dividend == -BigInt::from(2u8).pow(255) && divisor == BigInt::from(-1) {
        -BigInt::from(2u8).pow(255)
    } else {
        let sign = get_sign(&dividend * &divisor);
        sign * (dividend.abs() / divisor.abs())
    };
    
    stack::push(&mut evm.stack, U256::new(quotient.to_u32_digits().1))?;
    
    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
