// /// 
// /// Ethereum Virtual Machine (EVM) Storage Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM storage related instructions.
// /// 
// use super::super::super::state::{get_storage, set_storage};
// use super::super::::{Evm};
// use super::super::gas::{GAS_SLOAD, GAS_STORAGE_CLEAR_REFUND, GAS_STORAGE_SET, GAS_STORAGE_UPDATE, charge_gas};
// use super::super::stack::{pop, push};
// /// 
// ///     Loads to the stack, the value corresponding to a certain key from the
// ///     storage of the current account.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn sload(evm: Evm) -> Result<(), Error> {
//     key = pop(evm.stack)?.to_be_bytes32()?;
//     charge_gas(evm, GAS_SLOAD)?;
//     value = get_storage(evm.env.state, evm.message.current_target, key)?;
//     push(evm.stack, value)?;
//     evm.pc += 1;
// }


// /// 
// ///     Stores a value at a certain key in the current context's storage.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// /// 
// ///     
// pub fn sstore(evm: Evm) -> Result<(), Error> {
//     key = pop(evm.stack)?.to_be_bytes32()?;
//     new_value = pop(evm.stack)?;
//     current_value = get_storage(evm.env.state, evm.message.current_target, key)?;
//     if new_value != 0 && current_value == 0 {
//         gas_cost = GAS_STORAGE_SET;
//     } else {
//         gas_cost = GAS_STORAGE_UPDATE;
//     }
//     charge_gas(evm, gas_cost)?;
//     if new_value == 0 && current_value != 0 {
//         evm.refund_counter += GAS_STORAGE_CLEAR_REFUND;
//     }
//     set_storage(evm.env.state, evm.message.current_target, key, new_value)?;
//     evm.pc += 1;
// }


