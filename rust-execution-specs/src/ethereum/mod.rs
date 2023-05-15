//!
//! # Ethereum Specification
//!
//! Seeing as internet connections have been vastly expanding across the
//! world, spreading information has become as cheap as ever. Bitcoin, for
//! example, has demonstrated the possibility of creating a decentralized,
//! trade system that is accessible around the world. Namecoin is another
//! system that built off of Bitcoin's currency structure to create other
//! simple technological applications.
//!
//! Ethereum's goal is to create a cryptographically secure system in which
//! any and all types of transaction-based concepts can be built. It provides
//! an exceptionally accessible and decentralized system to build software
//! and execute transactions.
//!
//! This package contains a reference implementation, written as simply as
//! possible, to aid in defining the behavior of Ethereum clients.
//!

pub mod base_types;
pub mod exceptions;
pub mod rlp;
pub mod genesis;
pub mod ethash;

// Helpers
pub mod utils;

// Hardforks
pub mod frontier;
