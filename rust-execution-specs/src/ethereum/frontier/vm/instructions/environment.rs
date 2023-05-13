// /// 
// /// Ethereum Virtual Machine (EVM) Environmental Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM environment related instructions.
// /// 
// use ::ethereum::base_types::{U256, Uint};
// use ::ethereum::utils::numeric::{ceil32};
// use super::super::super::state::{get_account};
// use super::super::super::utils::address::{to_address};
// use super::super::super::vm::memory::{buffer_read, memory_write};
// use super::super::::{Evm};
// use super::super::gas::{GAS_BALANCE, GAS_BASE, GAS_COPY, GAS_EXTERNAL, GAS_VERY_LOW, calculate_gas_extend_memory, charge_gas};
// use super::super::stack::{pop, push};
// /// 
// ///     Pushes the address of the current executing account to the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn address(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256.from_be_bytes(evm.message.current_target)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Pushes the balance of the given account onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn balance(evm: Evm) -> Result<(), Error> {
//     address = to_address(pop(evm.stack)?)?;
//     charge_gas(evm, GAS_BALANCE)?;
//     balance = get_account(evm.env.state, address)?.balance;
//     push(evm.stack, balance)?;
//     evm.pc += 1;
// }


// /// 
// ///     Pushes the address of the original transaction sender to the stack.
// ///     The origin address can only be an EOA.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn origin(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256.from_be_bytes(evm.env.origin)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Pushes the address of the caller onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn caller(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256.from_be_bytes(evm.message.caller)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the value (in wei) sent with the call onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn callvalue(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, evm.message.value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push a word (32 bytes) of the input data belonging to the current
// ///     environment onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn calldataload(evm: Evm) -> Result<(), Error> {
//     start_index = pop(evm.stack)?;
//     charge_gas(evm, GAS_VERY_LOW)?;
//     value = buffer_read(evm.message.data, start_index, U256(32)?)?;
//     push(evm.stack, U256.from_be_bytes(value)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the size of input data in current environment onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn calldatasize(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(len(evm.message.data)?)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Copy a portion of the input data in current environment to memory.
// /// 
// ///     This will also expand the memory, in case that the memory is insufficient
// ///     to store the data.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn calldatacopy(evm: Evm) -> Result<(), Error> {
//     memory_start_index = pop(evm.stack)?;
//     data_start_index = pop(evm.stack)?;
//     size = pop(evm.stack)?;
//     words = (ceil32(Uint(size)?)?).floordiv(32);
//     copy_gas_cost = GAS_COPY * words;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_index, size)])?;
//     charge_gas(evm, GAS_VERY_LOW + copy_gas_cost + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     value = buffer_read(evm.message.data, data_start_index, size)?;
//     memory_write(evm.memory, memory_start_index, value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the size of code running in current environment onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn codesize(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, U256(len(evm.code)?)?)?;
//     evm.pc += 1;
// }


// /// 
// ///     Copy a portion of the code in current environment to memory.
// /// 
// ///     This will also expand the memory, in case that the memory is insufficient
// ///     to store the data.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn codecopy(evm: Evm) -> Result<(), Error> {
//     memory_start_index = pop(evm.stack)?;
//     code_start_index = pop(evm.stack)?;
//     size = pop(evm.stack)?;
//     words = (ceil32(Uint(size)?)?).floordiv(32);
//     copy_gas_cost = GAS_COPY * words;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_index, size)])?;
//     charge_gas(evm, GAS_VERY_LOW + copy_gas_cost + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     value = buffer_read(evm.code, code_start_index, size)?;
//     memory_write(evm.memory, memory_start_index, value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the gas price used in current environment onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn gasprice(evm: Evm) -> Result<(), Error> {
//     // pass;
//     charge_gas(evm, GAS_BASE)?;
//     push(evm.stack, evm.env.gas_price)?;
//     evm.pc += 1;
// }


// /// 
// ///     Push the code size of a given account onto the stack.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn extcodesize(evm: Evm) -> Result<(), Error> {
//     address = to_address(pop(evm.stack)?)?;
//     charge_gas(evm, GAS_EXTERNAL)?;
//     codesize = U256(len(get_account(evm.env.state, address)?.code)?)?;
//     push(evm.stack, codesize)?;
//     evm.pc += 1;
// }


// /// 
// ///     Copy a portion of an account's code to memory.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn extcodecopy(evm: Evm) -> Result<(), Error> {
//     address = to_address(pop(evm.stack)?)?;
//     memory_start_index = pop(evm.stack)?;
//     code_start_index = pop(evm.stack)?;
//     size = pop(evm.stack)?;
//     words = (ceil32(Uint(size)?)?).floordiv(32);
//     copy_gas_cost = GAS_COPY * words;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_index, size)])?;
//     charge_gas(evm, GAS_EXTERNAL + copy_gas_cost + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     code = get_account(evm.env.state, address)?.code;
//     value = buffer_read(code, code_start_index, size)?;
//     memory_write(evm.memory, memory_start_index, value)?;
//     evm.pc += 1;
// }


