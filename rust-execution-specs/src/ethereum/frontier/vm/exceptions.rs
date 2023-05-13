//! Ethereum Virtual Machine (EVM) Exceptions
//! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! Introduction
//! ------------
//!
//! Exceptions which cause the EVM to halt exceptionally.

pub type Result<T, E = EvmError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum EvmError {
    /// Indicates that the EVM has experienced an exceptional halt. This causes
    /// execution to immediately end with all gas being consumed.
    Halt,
    /// Occurs when a pop is executed on an empty stack.
    StackUnderflow,
    /// Occurs when a push is executed on a stack at max capacity.
    StackOverflow,
    /// Occurs when an operation costs more than the amount of gas left in the
    /// frame.
    OutOfGas,
    /// Raised when an invalid opcode is encountered.
    InvalidOpcode,
    /// Occurs when the destination of a jump operation doesn't meet any of the
    /// following criteria:
    ///
    ///   * The jump destination is less than the length of the code.
    ///   * The jump destination should have the `JUMPDEST` opcode (0x5B).
    ///   * The jump destination shouldn't be part of the data corresponding to
    ///     `PUSH-N` opcodes.
    InvalidJumpDest,
    /// Raised when the message depth is greater than `1024`
    StackDepthLimit,
}
