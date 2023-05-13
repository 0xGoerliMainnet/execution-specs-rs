//!
//! # Exceptions
//!
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//!
//! ## Introduction
//!
//! The Ethereum specification exception classes.
//!
//!
//! The base class from which all exceptions thrown by the specification during
//! normal operation derive.
//!     

#[derive(Debug)]
pub enum EthereumException {
    ///
    ///     Thrown when a block being processed is found to be invalid.
    ///
    InvalidBlock,

    ///
    ///     Indicates that RLP decoding failed.
    ///
    RLPDecodingError,

    ///
    ///     Indicates that RLP encoding failed.
    ///
    RLPEncodingError,

    // Sundry pythonesque errors.
    ValueError,

    FileNotFound(String),

    JsonDecodeError(String),

    BadHexString(String),
}
