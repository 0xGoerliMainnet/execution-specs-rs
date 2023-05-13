use super::super::{
    exceptions::{Result},
    gas, stack, Evm,
};
use crate::ethereum::base_types::U256;
use crate::ethereum::utils::numeric::get_sign;
use num_bigint::BigInt;
use num_traits::Signed;

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
    gas::charge_gas(evm, gas::GAS_LOW())?;

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
    gas::charge_gas(evm, gas::GAS_LOW())?;

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
    // fixme: will not do sign conversion
    let dividend = BigInt::from(stack::pop(&mut evm.stack)?);
    let divisor = BigInt::from(stack::pop(&mut evm.stack)?);

    // GAS
    gas::charge_gas(evm, gas::GAS_LOW())?;

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

/// Modulo remainder of the top two elements of the stack. Pushes the result
/// back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn r#mod(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_LOW())?;

    // OPERATION
    let remainder = if y == U256::from(0u8) {
        U256::from(0u8)
    } else {
        x % y
    };

    stack::push(&mut evm.stack, remainder)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Signed modulo remainder of the top two elements of the stack. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn smod(evm: &mut Evm) -> Result<()> {
    // STACK
    // fixme: does not do sign conversion
    let x = BigInt::from(stack::pop(&mut evm.stack)?);
    let y = BigInt::from(stack::pop(&mut evm.stack)?);

    // GAS
    gas::charge_gas(evm, gas::GAS_LOW())?;

    // OPERATION
    let remainder = if y == BigInt::from(0u8) {
        BigInt::from(0u8)
    } else {
        get_sign(x.clone()) * (x.abs() % y.abs())
    };

    stack::push(&mut evm.stack, U256::new(remainder.to_u32_digits().1))?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Modulo addition of the top 2 elements with the 3rd element. Pushes the
/// result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn addmod(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;
    let z = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_MID())?;

    // OPERATION
    let result = if z == U256::from(0u8) {
        U256::from(0u8)
    } else {
        (x + y) % z
    };

    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Modulo multiplication of the top 2 elements with the 3rd element. Pushes
/// the result back on the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn mulmod(evm: &mut Evm) -> Result<()> {
    // STACK
    let x = stack::pop(&mut evm.stack)?;
    let y = stack::pop(&mut evm.stack)?;
    let z = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_MID())?;

    // OPERATION
    let result = if z == U256::from(0u8) {
        U256::from(0u8)
    } else {
        (x * y) % z
    };

    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Exponential operation of the top 2 elements. Pushes the result back on
/// the stack.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn exp(evm: &mut Evm) -> Result<()> {
    // STACK
    let base = stack::pop(&mut evm.stack)?;
    let exponent = stack::pop(&mut evm.stack)?;

    // GAS
    // This is equivalent to 1 + floor(log(y, 256)). But in python the log
    // function is inaccurate leading to wrong results.
    let exponent_bits = exponent.bits();
    let exponent_bytes = (exponent_bits + 7) / 8;
    gas::charge_gas(
        evm,
        gas::GAS_EXPONENTIATION() + gas::GAS_EXPONENTIATION_PER_BYTE() * U256::from(exponent_bytes),
    )?;

    // OPERATION
    // fixme: ceil
    let result = base.pow(u32::try_from(exponent).unwrap());

    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}

/// Sign extend operation. In other words, extend a signed number which
/// fits in N bytes to 32 bytes.
///
/// Parameters
/// ----------
/// evm :
///     The current EVM frame.
pub fn signextend(evm: &mut Evm) -> Result<()> {
    // STACK
    let byte_num = stack::pop(&mut evm.stack)?;
    let value = stack::pop(&mut evm.stack)?;

    // GAS
    gas::charge_gas(evm, gas::GAS_LOW())?;

    // OPERATION
    let result = if byte_num > U256::from(31u8) {
        // Can't extend any further
        value
    } else {
        // U256(0).to_be_bytes() gives b'' instead b'\x00'. (fixme: how does to_bytes_be handle 0)
        let value_bytes = value.to_bytes_be();
        // Now among the obtained value bytes, consider only
        // N `least significant bytes`, where N is `byte_num + 1`.
        let value_bytes = &value_bytes[(31 - usize::try_from(byte_num.clone()).unwrap())..];
        let sign_bit = value_bytes[0] >> 7;

        if sign_bit == 0 {
            U256::from_bytes_be(value_bytes)
        } else {
            let num_bytes_prepend = U256::from(32u8) - (byte_num + U256::from(1u8));
            let mut bytes = [0xff].repeat(usize::try_from(num_bytes_prepend).unwrap());
            bytes.extend(value_bytes);
            U256::from_bytes_be(&bytes)
        }
    };

    stack::push(&mut evm.stack, result)?;

    // PROGRAM COUNTER
    evm.pc += 1;
    Ok(())
}
