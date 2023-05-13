// /// 
// /// Ethereum Virtual Machine (EVM) Logging Instructions
// /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// Implementations of the EVM logging instructions.
// /// 
// use ::functools::{partial};
// use ::ethereum::base_types::{U256};
// use super::super::super::fork_types::{Log};
// use super::super::::{Evm};
// use super::super::gas::{GAS_LOG, GAS_LOG_DATA, GAS_LOG_TOPIC, calculate_gas_extend_memory, charge_gas};
// use super::super::memory::{memory_read_bytes};
// use super::super::stack::{pop};
// /// 
// ///     Appends a log entry, having `num_topics` topics, to the evm logs.
// /// 
// ///     This will also expand the memory if the data (required by the log entry)
// ///     corresponding to the memory is not accessible.
// /// 
// ///     Parameters
// ///     ----------
// ///     evm :
// ///         The current EVM frame.
// ///     num_topics :
// ///         The number of topics to be included in the log entry.
// /// 
// ///     
// pub fn log_n(evm: Evm, num_topics: U256) -> Result<(), Error> {
//     memory_start_index = pop(evm.stack)?;
//     size = pop(evm.stack)?;
//     topics = [];
//     for _ in range(num_topics)? {
//         topic = pop(evm.stack)?.to_be_bytes32()?;
//         topics.append(topic)?;
//     }
//     extend_memory = calculate_gas_extend_memory(evm.memory, [(memory_start_index, size)])?;
//     charge_gas(evm, GAS_LOG + GAS_LOG_DATA * size + GAS_LOG_TOPIC * num_topics + extend_memory.cost)?;
//     evm.memory += [0] * extend_memory.expand_by;
//     log_entry = Log(address = evm.message.current_target, topics = tuple(topics)?, data = memory_read_bytes(evm.memory, memory_start_index, size)?)?;
//     evm.logs = evm.logs + (log_entry);
//     evm.pc += 1;
// }


// log0 = partial(log_n, num_topics = 0)?;
// log1 = partial(log_n, num_topics = 1)?;
// log2 = partial(log_n, num_topics = 2)?;
// log3 = partial(log_n, num_topics = 3)?;
// log4 = partial(log_n, num_topics = 4)?;
