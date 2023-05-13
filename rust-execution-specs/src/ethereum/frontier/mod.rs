//! 
//! # Ethereum Frontier Hardfork
//! 
//! The first Ethereum hardfork.
//! 


/// Where it all started.
pub const MAINNET_FORK_BLOCK : u32 = 0;

pub mod fork_types;
pub mod trie;
pub mod bloom;
pub mod fork;
pub mod state;
pub mod vm;
