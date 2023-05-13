// /// 
// /// Ethereum Virtual Machine (EVM) System Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM system related instructions.
// /// 
// use ::ethereum::base_types::{U256, Bytes0, Uint};
// use super::super::super::fork_types::{Address};
// use super::super::super::state::{account_has_code_or_nonce, get_account, increment_nonce, set_account_balance};
// use super::super::super::utils::address::{compute_contract_address, to_address};
// use super::super::::{Evm, Message, incorporate_child_on_error, incorporate_child_on_success};
// use super::super::gas::{GAS_CREATE, GAS_ZERO, calculate_gas_extend_memory, calculate_message_call_gas, charge_gas};
// use super::super::memory::{memory_read_bytes, memory_write};
// use super::super::stack::{pop, push};
// /// 
// ///     Creates a new account with associated code.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn create(evm: Evm) -> Result<(), Error> {
//     use super::super::super::vm::interpreter::{STACK_DEPTH_LIMIT, process_create_message};
//     endowment = pop(evm.stack)?;
//     memory_start_position = pop(evm.stack)?;
//     memory_size = pop(evm.stack)?;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_position, memory_size)])?;
//     charge_gas(evm, GAS_CREATE + extend_memory.cost)?;
//     create_message_gas = evm.gas_left;
//     evm.gas_left = U256(0)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     sender_address = evm.message.current_target;
//     sender = get_account(evm.env.state, sender_address)?;
//     contract_address = compute_contract_address(evm.message.current_target, get_account(evm.env.state, evm.message.current_target)?.nonce)?;
//     if sender.balance < endowment || sender.nonce == Uint((2).pow(64) - 1)? || evm.message.depth + 1 > STACK_DEPTH_LIMIT {
//         push(evm.stack, U256(0)?)?;
//         evm.gas_left += create_message_gas;
//     } else if account_has_code_or_nonce(evm.env.state, contract_address)? {
//         increment_nonce(evm.env.state, evm.message.current_target)?;
//         push(evm.stack, U256(0)?)?;
//     } else {
//         call_data = memory_read_bytes(evm.memory, memory_start_position, memory_size)?;
//         increment_nonce(evm.env.state, evm.message.current_target)?;
//         child_message = Message(caller = evm.message.current_target, target = Bytes0()?, gas = create_message_gas, value = endowment, data = [], code = call_data, current_target = contract_address, depth = evm.message.depth + 1, code_address = ())?;
//         child_evm = process_create_message(child_message, evm.env)?;
//         if child_evm.has_erred {
//             incorporate_child_on_error(evm, child_evm)?;
//             push(evm.stack, U256(0)?)?;
//         } else {
//             incorporate_child_on_success(evm, child_evm)?;
//             push(evm.stack, U256.from_be_bytes(child_evm.message.current_target)?)?;
//         }
//     }
//     evm.pc += 1;
// }


// /// 
// ///     Halts execution returning output data.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn return_(evm: Evm) -> Result<(), Error> {
//     memory_start_position = pop(evm.stack)?;
//     memory_size = pop(evm.stack)?;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_position, memory_size)])?;
//     charge_gas(evm, GAS_ZERO + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     evm.output = memory_read_bytes(evm.memory, memory_start_position, memory_size)?;
//     evm.running = false;
//     // pass;
// }


// /// 
// ///     Perform the core logic of the `CALL*` family of opcodes.
// ///     
// pub fn generic_call(evm: Evm, gas: Uint, value: U256, caller: Address, to: Address, code_address: Address, memory_input_start_position: U256, memory_input_size: U256, memory_output_start_position: U256, memory_output_size: U256) -> Result<(), Error> {
//     use super::super::super::vm::interpreter::{STACK_DEPTH_LIMIT, process_message};
//     if evm.message.depth + 1 > STACK_DEPTH_LIMIT {
//         evm.gas_left += gas;
//         push(evm.stack, U256(0)?)?;
//         return;
//     }
//     call_data = memory_read_bytes(evm.memory, memory_input_start_position, memory_input_size)?;
//     code = get_account(evm.env.state, code_address)?.code;
//     child_message = Message(caller = caller, target = to, gas = U256(gas)?, value = value, data = call_data, code = code, current_target = to, depth = evm.message.depth + 1, code_address = code_address)?;
//     child_evm = process_message(child_message, evm.env)?;
//     if child_evm.has_erred {
//         incorporate_child_on_error(evm, child_evm)?;
//         push(evm.stack, U256(0)?)?;
//     } else {
//         incorporate_child_on_success(evm, child_evm)?;
//         push(evm.stack, U256(1)?)?;
//     }
//     actual_output_size = min(memory_output_size, U256(len(child_evm.output)?)?)?;
//     memory_write(evm.memory, memory_output_start_position, child_evm.output[..actual_output_size])?;
// }


// /// 
// ///     Message-call into an account.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn call(evm: Evm) -> Result<(), Error> {
//     gas = Uint(pop(evm.stack)?)?;
//     to = to_address(pop(evm.stack)?)?;
//     value = pop(evm.stack)?;
//     memory_input_start_position = pop(evm.stack)?;
//     memory_input_size = pop(evm.stack)?;
//     memory_output_start_position = pop(evm.stack)?;
//     memory_output_size = pop(evm.stack)?;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_input_start_position, memory_input_size), (memory_output_start_position, memory_output_size)])?;
//     message_call_gas = calculate_message_call_gas(evm.env.state, gas, to, value)?;
//     charge_gas(evm, message_call_gas.cost + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     sender_balance = get_account(evm.env.state, evm.message.current_target)?.balance;
//     if sender_balance < value {
//         push(evm.stack, U256(0)?)?;
//         evm.gas_left += message_call_gas.stipend;
//     } else {
//         generic_call(evm, message_call_gas.stipend, value, evm.message.current_target, to, to, memory_input_start_position, memory_input_size, memory_output_start_position, memory_output_size)?;
//     }
//     evm.pc += 1;
// }


// /// 
// ///     Message-call into this account with alternative account’s code.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn callcode(evm: Evm) -> Result<(), Error> {
//     gas = Uint(pop(evm.stack)?)?;
//     code_address = to_address(pop(evm.stack)?)?;
//     value = pop(evm.stack)?;
//     memory_input_start_position = pop(evm.stack)?;
//     memory_input_size = pop(evm.stack)?;
//     memory_output_start_position = pop(evm.stack)?;
//     memory_output_size = pop(evm.stack)?;
//     to = evm.message.current_target;
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_input_start_position, memory_input_size), (memory_output_start_position, memory_output_size)])?;
//     message_call_gas = calculate_message_call_gas(evm.env.state, gas, to, value)?;
//     charge_gas(evm, message_call_gas.cost + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     sender_balance = get_account(evm.env.state, evm.message.current_target)?.balance;
//     if sender_balance < value {
//         push(evm.stack, U256(0)?)?;
//         evm.gas_left += message_call_gas.stipend;
//     } else {
//         generic_call(evm, message_call_gas.stipend, value, evm.message.current_target, to, code_address, memory_input_start_position, memory_input_size, memory_output_start_position, memory_output_size)?;
//     }
//     evm.pc += 1;
// }


// /// 
// ///     Halt execution and register account for later deletion.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     
// pub fn selfdestruct(evm: Evm) -> Result<(), Error> {
//     beneficiary = to_address(pop(evm.stack)?)?;
//     // pass;
//     originator = evm.message.current_target;
//     beneficiary_balance = get_account(evm.env.state, beneficiary)?.balance;
//     originator_balance = get_account(evm.env.state, originator)?.balance;
//     set_account_balance(evm.env.state, beneficiary, beneficiary_balance + originator_balance)?;
//     set_account_balance(evm.env.state, originator, U256(0)?)?;
//     evm.accounts_to_delete.add(originator)?;
//     evm.running = false;
//     // pass;
// }


