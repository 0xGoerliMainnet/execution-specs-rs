// /// 
// /// Ethereum Virtual Machine (EVM) Interpreter
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// A straightforward interpreter that executes EVM code.
// /// 
// use ::dataclasses::{dataclass};
// use ::typing::{Set, Tuple};
// use ::ethereum::{evm_trace};
// use ::ethereum::base_types::{U256, Bytes0, Uint};
// use super::super::fork_types::{Address, Log};
// use super::super::state::{account_has_code_or_nonce, begin_transaction, commit_transaction, move_ether, rollback_transaction, set_code, touch_account};
// use super::super::vm::{Message};
// use super::super::vm::gas::{GAS_CODE_DEPOSIT, REFUND_SELF_DESTRUCT, charge_gas};
// use super::super::vm::precompiled_contracts::mapping::{PRE_COMPILED_CONTRACTS};
// use super::::{Environment, Evm};
// use super::exceptions::{ExceptionalHalt, InvalidOpcode, StackDepthLimitError};
// use super::instructions::{Ops, op_implementation};
// use super::runtime::{get_valid_jump_destinations};
// STACK_DEPTH_LIMIT = U256(1024)?;
// /// 
// ///     Output of a particular message call
// /// 
// ///     Contains the following:
// /// 
// ///           1. `gas_left`: remaining gas after execution.
// ///           2. `refund_counter`: gas to refund after execution.
// ///           3. `logs`: list of `Log` generated during execution.
// ///           4. `accounts_to_delete`: Contracts which have self-destructed.
// ///           5. `has_erred`: True if execution has caused an error.
// ///     
// struct MessageCallOutput {
//     gas_left: U256,
//     refund_counter: U256,
//     logs: Tuple[Log][...],
//     accounts_to_delete: Set[Address],
//     has_erred: bool,
// }


// impl MessageCallOutput {
// }


// /// 
// ///     If `message.current` is empty then it creates a smart contract
// ///     else it executes a call from the `message.caller` to the `message.target`.
// /// 
// ///     Parameters
// ///     ----------
// ///     message :
// ///         Transaction specific items.
// /// 
// ///     env :
// ///         External items required for EVM execution.
// /// 
// ///     Returns
// ///     -------
// ///     output : `MessageCallOutput`
// ///         Output of the message call
// ///     
// pub fn process_message_call(message: Message, env: Environment) -> Result<MessageCallOutput, Error> {
//     if message.target == Bytes0([])? {
//         is_collision = account_has_code_or_nonce(env.state, message.current_target)?;
//         if is_collision {
//             return Ok(MessageCallOutput(U256(0)?, U256(0)?, tuple()?, set()?, true)?);
//         } else {
//             evm = process_create_message(message, env)?;
//         }
//     } else {
//         evm = process_message(message, env)?;
//     }
//     if evm.has_erred {
//         // TypedAssignment unsupported
//         accounts_to_delete = set()?;
//         refund_counter = U256(0)?;
//     } else {
//         logs = evm.logs;
//         accounts_to_delete = evm.accounts_to_delete;
//         refund_counter = evm.refund_counter + REFUND_SELF_DESTRUCT * len(evm.accounts_to_delete)?;
//     }
//     return Ok(MessageCallOutput(gas_left = evm.gas_left, refund_counter = refund_counter, logs = logs, accounts_to_delete = accounts_to_delete, has_erred = evm.has_erred)?);
// }


// /// 
// ///     Executes a call to create a smart contract.
// /// 
// ///     Parameters
// ///     ----------
// ///     message :
// ///         Transaction specific items.
// ///     env :
// ///         External items required for EVM execution.
// /// 
// ///     Returns
// ///     -------
// ///     evm: :py:class:`~ethereum.frontier.vm.Evm`
// ///         Items containing execution specific objects.
// ///     
// pub fn process_create_message(message: Message, env: Environment) -> Result<Evm, Error> {
//     evm = process_message(message, env)?;
//     if !(evm.has_erred) {
//         contract_code = evm.output;
//         contract_code_gas = len(contract_code)? * GAS_CODE_DEPOSIT;
//         // Try 
//             charge_gas(evm, contract_code_gas)?;
//     }
//     return Ok(evm);
// }


// /// 
// ///     Executes a call to create a smart contract.
// /// 
// ///     Parameters
// ///     ----------
// ///     message :
// ///         Transaction specific items.
// ///     env :
// ///         External items required for EVM execution.
// /// 
// ///     Returns
// ///     -------
// ///     evm: :py:class:`~ethereum.frontier.vm.Evm`
// ///         Items containing execution specific objects
// ///     
// pub fn process_message(message: Message, env: Environment) -> Result<Evm, Error> {
//     if message.depth > STACK_DEPTH_LIMIT {
//         return Err(Error::StackDepthLimitError("Stack depth limit reached")?);
//     }
//     begin_transaction(env.state)?;
//     touch_account(env.state, message.current_target)?;
//     if message.value != 0 {
//         move_ether(env.state, message.caller, message.current_target, message.value)?;
//     }
//     evm = execute_code(message, env)?;
//     if evm.has_erred {
//         rollback_transaction(env.state)?;
//     } else {
//         commit_transaction(env.state)?;
//     }
//     return Ok(evm);
// }


// /// 
// ///     Executes bytecode present in the `message`.
// /// 
// ///     Parameters
// ///     ----------
// ///     message :
// ///         Transaction specific items.
// ///     env :
// ///         External items required for EVM execution.
// /// 
// ///     Returns
// ///     -------
// ///     evm: `ethereum.vm.EVM`
// ///         Items containing execution specific objects
// ///     
// pub fn execute_code(message: Message, env: Environment) -> Result<Evm, Error> {
//     code = message.code;
//     valid_jump_destinations = get_valid_jump_destinations(code)?;
//     evm = Evm(pc = Uint(0)?, stack = [], memory = bytearray()?, code = code, gas_left = message.gas, env = env, valid_jump_destinations = valid_jump_destinations, logs = (), refund_counter = U256(0)?, running = true, message = message, output = [], accounts_to_delete = set()?, has_erred = false)?;
//     // Try 
//         if (evm.message.code_address).contains(PRE_COMPILED_CONTRACTS) {
//             evm_trace(evm, evm.message.code_address)?;
//             PRE_COMPILED_CONTRACTS[evm.message.code_address](evm)?;
//             return Ok(evm);
//         }
//         while evm.running && evm.pc < len(evm.code)? {
//             // Try 
//                 op = Ops(evm.code[evm.pc])?;
//             evm_trace(evm, op)?;
//             op_implementation[op](evm)?;
//         }
//     return Ok(evm);
// }


