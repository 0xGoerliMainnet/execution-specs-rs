//! 
//! State Trie
//! ^^^^^^^^^^
//! 
//! .. contents:: Table of Contents
//!     :backlinks: none
//!     :local:
//! 
//! Introduction
//! ------------
//! 
//! The state trie is the structure responsible for storing
//! `.fork_types.Account` objects.
//! 
#![allow(dead_code)]

// NOTE: Import::Import unsupported
// use ::dataclasses::{dataclass, field};
// use ::typing::{Callable, Dict, Generic, List, Mapping, MutableMapping, Optional, Sequence, TypeVar, Union, cast};
// use ::ethereum::crypto::hash::{keccak256};
// use ::ethereum::utils::ensure::{ensure};
// use ::ethereum::utils::hexadecimal::{hex_to_bytes};
// use super::super::::{rlp};
// use super::super::base_types::{U256, Bytes, Uint, slotted_freezable};
// use super::fork_types::{Account, Address, Receipt, Root, Transaction, encode_account};

use std::collections::HashMap;

use hex_literal::hex;

use crate::ethereum::{rlp::{EncodeRlp, encode_sequence}, base_types::{Bytes, U256, Bytes32}, exceptions::EthereumException};

use super::fork_types::{keccak256, Account, Address};

pub type Root = [u8; 32];

pub trait Key : Eq + std::hash::Hash + AsRef<[u8]> {}

pub const EMPTY_TRIE_ROOT : Root = hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

pub trait Node: PartialEq + std::fmt::Debug + Clone {
    fn encode<F : Fn(&Address) -> Root>(&self, f: F) -> Bytes;
}

impl Node for String {
    fn encode<F : Fn(&Address) -> Root>(&self, _f: F) -> Bytes {
        self.as_bytes().into()
    }
}

impl Node for Option<Account> {
    fn encode<F : Fn(&Address) -> Root>(&self, _f: F) -> Bytes {
        Bytes::default()
    }
}

impl Node for U256 {
    fn encode<F : Fn(&Address) -> Root>(&self, _f: F) -> Bytes {
        Bytes::default()
    }
}

impl Key for String {
}

impl Key for Address {
}

impl Key for Bytes32 {
    
}

// pub enum Node {
//     Account(Account, Root),
//     Bytes(Bytes),
//     Transaction(Transaction),
//     Receipt(Receipt),
//     Uint(Uint),
//     U256(U256),
//     String(String),
//     Null(()),
// }

/// Leaf node in the Merkle Trie
pub struct LeafNode<T : EncodeRlp> {
    rest_of_key: Bytes,
    value: T,
}


/// Extension node in the Merkle Trie
pub struct ExtensionNode<T : EncodeRlp> {
    key_segment: Bytes,
    subnode: Box<InternalNode<T>>,
}


/// Branch node in the Merkle Trie
pub struct BranchNode<T : EncodeRlp> {
    subnodes: Vec<InternalNode<T>>,
    value: T,
}

pub enum InternalNode<T : EncodeRlp> {
    LeafNode(LeafNode<T>),
    ExtensionNode(ExtensionNode<T>),
    BranchNode(BranchNode<T>),
    Null,
}
/// 
///     Encodes a Merkle Trie node into its RLP form. The RLP will then be
///     serialized into a `Bytes` and hashed unless it is less that 32 bytes
///     when serialized.
/// 
///     This function also accepts `None`, representing the absence of a node,
///     which is encoded to `b""`.
/// 
///     Parameters
///     ----------
///     node : Optional[InternalNode]
///         The node to encode.
/// 
///     Returns
///     -------
///     encoded : `rlp.RLP`
///         The node encoded as RLP.
///     
pub fn encode_internal_node<V : EncodeRlp>(node: &InternalNode<V>) -> Bytes {
    let mut encodes = vec![];
    match node {
        InternalNode::LeafNode(node) => {
            encodes.extend_from_slice(nibble_list_to_compact(&node.rest_of_key, true).encode().as_ref());
            encodes.extend_from_slice(node.value.encode().as_ref());
        },
        InternalNode::ExtensionNode(node) => {
            encodes.extend_from_slice(nibble_list_to_compact(&node.key_segment, false).encode().as_ref());
            encodes.extend_from_slice(encode_internal_node(&node.subnode).encode().as_ref());
        },
        InternalNode::BranchNode(node) => {
            for s in &node.subnodes {
                encodes.extend_from_slice(encode_internal_node(s).encode().as_ref());
            }
            encodes.extend_from_slice(node.value.encode().as_ref());
        },
        InternalNode::Null => {
            encodes.extend_from_slice([0_u8; 0].encode().as_ref());
        },
    };

    let encoded = encode_sequence(&encodes);
    if encoded.len() < 32 {
        encoded
    } else {
        Bytes::from(keccak256(&encoded))
    }
}

// pub fn (node: InternalNode) -> Bytes {
//     if (node).is(()) {
//         unencoded = [];
//     } else if isinstance(node, LeafNode)? {
//         unencoded = (nibble_list_to_compact(node.rest_of_key, true)?, node.value);
//     } else if isinstance(node, ExtensionNode)? {
//         unencoded = (nibble_list_to_compact(node.key_segment, false)?, node.subnode);
//     } else if isinstance(node, BranchNode)? {
//         unencoded = node.subnodes + [node.value];
//     } else {
//         return Err(Error::AssertionError("Invalid internal node type {type(node)}!")?);
//     }
//     encoded = rlp.encode(unencoded)?;
//     if len(encoded)? < 32 {
//         return Ok(unencoded);
//     } else {
//         return Ok(keccak256(encoded)?);
//     }
// }


/// 
///     Encode a Node for storage in the Merkle Trie.
/// 
///     Currently mostly an unimplemented stub.
///     
pub fn encode_node<N : Node, F : Fn(&Address) -> Root>(node: &N, f: F) -> Bytes {
    node.encode(f)
}

/// 
///     The Merkle Trie.
///     
#[derive(Clone, Debug)]
pub struct Trie<K, V>
where
    K: Key, V: Node,
{
    pub secured: bool,
    pub default: V,
    pub data: HashMap<K, V>,
}


impl<K, V> Trie<K, V>
where
    K: Key, V: Node,
{
    pub fn new(secured: bool, default: V) -> Self {
        Self {
            secured,
            default,
            data: HashMap::new(),
        }
    }
}


// /// 
// ///     Create a copy of `trie`. Since only frozen objects may be stored in tries,
// ///     the contents are reused.
// /// 
// ///     Parameters
// ///     ----------
// ///     trie: `Trie`
// ///         Trie to copy.
// /// 
// ///     Returns
// ///     -------
// ///     new_trie : `Trie[K, V]`
// ///         A copy of the trie.
// ///     
// pub fn copy_trie<K, V>(trie: Trie<K, V>) -> Result<Trie<K, V>, EthereumException> {
//     return Ok(Trie(trie.secured, trie.default, copy.copy(trie._data)?)?);
// }


/// 
///     Stores an item in a Merkle Trie.
/// 
///     This method deletes the key if `value == trie.default`, because the Merkle
///     Trie represents the default value by omitting it from the trie.
/// 
///     Parameters
///     ----------
///     trie: `Trie`
///         Trie to store in.
///     key : `Bytes`
///         Key to lookup.
///     value : `V`
///         Node to insert at `key`.
///     
pub fn trie_set<K, V>(trie: &mut Trie<K, V>, key: K, value: V)
where
    K: Key, V: Node,
{
    if value == trie.default {
        if trie.data.contains_key(&key) {
            trie.data.remove(&key);
        }
    } else {
        trie.data.insert(key, value);
    }
}


/// 
///     Gets an item from the Merkle Trie.
/// 
///     This method returns `trie.default` if the key is missing.
/// 
///     Parameters
///     ----------
///     trie:
///         Trie to lookup in.
///     key :
///         Key to lookup.
/// 
///     Returns
///     -------
///     node : `V`
///         Node at `key` in the trie.
///     
pub fn trie_get<'t, K, V>(trie: &'t Trie<K, V>, key: &K) -> V
where
    K: Key, V: Node,
{
    trie.data.get(key).cloned().unwrap_or_else(|| trie.default.clone())
}


// /// 
// ///     Find the longest common prefix of two sequences.
// ///     
// pub fn common_prefix_length(a: Sequence, b: Sequence) -> Result<int, EthereumException> {
//     for i in range(len(a)?)? {
//         if i >= len(b)? || a[i] != b[i] {
//             return Ok(i);
//         }
//     }
//     return Ok(len(a)?);
// }


// /// 
// ///     Compresses nibble-list into a standard byte array with a flag.
// /// 
// ///     A nibble-list is a list of byte values no greater than `15`. The flag is
// ///     encoded in high nibble of the highest byte. The flag nibble can be broken
// ///     down into two two-bit flags.
// /// 
// ///     Highest nibble::
// /// 
// ///         +---+---+----------+--------+
// ///         | _ | _ | is_leaf | parity |
// ///         +---+---+----------+--------+
// ///           3   2      1         0
// /// 
// /// 
// ///     The lowest bit of the nibble encodes the parity of the length of the
// ///     remaining nibbles -- `0` when even and `1` when odd. The second lowest bit
// ///     is used to distinguish leaf and extension nodes. The other two bits are not
// ///     used.
// /// 
// ///     Parameters
// ///     ----------
// ///     x :
// ///         Array of nibbles.
// ///     is_leaf :
// ///         True if this is part of a leaf node, or false if it is an extension
// ///         node.
// /// 
// ///     Returns
// ///     -------
// ///     compressed : `bytearray`
// ///         Compact byte array.
// ///     
pub fn nibble_list_to_compact(x: &[u8], is_leaf: bool) -> Bytes {
    let mut compact = vec![];
    if x.len() % 2 == 0 {
        compact.push(16 * 2 * is_leaf as u8);
        for i in (0..x.len()).step_by(2) {
            compact.push(16 * x[i] + x[i + 1]);
        }
    } else {
        compact.push(16 * 2 * is_leaf as u8 + 1 + x[0]);
        for i in (1..x.len()).step_by(2) {
            compact.push(16 * x[i] + x[i + 1]);
        }
    }
    Box::from(compact)
}


// /// 
// ///     Converts a `Bytes` into to a sequence of nibbles (bytes with value < 16).
// /// 
// ///     Parameters
// ///     ----------
// ///     bytes_:
// ///         The `Bytes` to convert.
// /// 
// ///     Returns
// ///     -------
// ///     nibble_list : `Bytes`
// ///         The `Bytes` in nibble-list format.
// ///     
// pub fn bytes_to_nibble_list(bytes_: Bytes) -> Result<Bytes, EthereumException> {
//     nibble_list = bytearray(2 * len(bytes_)?)?;
//     for (byte_index, byte) in enumerate(bytes_)? {
//         nibble_list[byte_index * 2] = byte & 240 >> 4;
//         nibble_list[byte_index * 2 + 1] = byte & 15;
//     }
//     return Ok(Bytes(nibble_list)?);
// }


/// 
///     Prepares the trie for root calculation. Removes values that are empty,
///     hashes the keys (if `secured == True`) and encodes all the nodes.
/// 
///     Parameters
///     ----------
///     trie :
///         The `Trie` to prepare.
///     get_storage_root :
///         Function to get the storage root of an account. Needed to encode
///         `Account` objects.
/// 
///     Returns
///     -------
///     out : `Mapping[ethereum.base_types.Bytes, Node]`
///         Object with keys mapped to nibble-byte form.
///     
pub fn _prepare_trie<K, V, F>(trie: &Trie<K, V>, f: F) -> Result<Vec<(Bytes, Bytes)>, EthereumException>
where
    K: Key, V: Node,
    F : Fn(&Address) -> Root + Clone,
{
    let mut res = vec![];
    for (_preimage, value) in &trie.data {
        let f = f.clone();
        if trie.secured {
            let encoded_value = encode_node(value, f);
            assert!(!encoded_value.is_empty());
            // res.push((bytes_to_nibble_list(&keccak256(preimage.as_ref())), encoded_value));
        } else {
            // res.push((bytes_to_nibble_list(&keccak256(preimage.as_ref())), preimage.as_ref()));
        };
    }
    res.sort();
    Ok(res)
}

/// 
///     Computes the root of a modified merkle patricia trie (MPT).
/// 
///     Parameters
///     ----------
///     trie :
///         `Trie` to get the root of.
///     get_storage_root :
///         Function to get the storage root of an account. Needed to encode
///         `Account` objects.
/// 
/// 
///     Returns
///     -------
///     root : `.fork_types.Root`
///         MPT root of the underlying key-value pairs.
///     
pub fn root<K, V, F : Fn(&Address) -> Root + Clone>(trie: &Trie<K, V>, f: F) -> Root
where
    K: Key, V: Node,
{
    let _obj = _prepare_trie(&trie, f).unwrap();
    todo!();
    // root_node = encode_internal_node(patricialize(obj, Uint(0)?)?)?;
    // if len(rlp.encode(root_node)?)? < 32 {
    //     return Ok(keccak256(rlp.encode(root_node)?)?);
    // } else {
    //     assert!(isinstance(root_node, Bytes)?);
    //     return Ok(Root(root_node)?);
    // }
}


// /// 
// ///     Structural composition function.
// /// 
// ///     Used to recursively patricialize and merkleize a dictionary. Includes
// ///     memoization of the tree structure and hashes.
// /// 
// ///     Parameters
// ///     ----------
// ///     obj :
// ///         Underlying trie key-value pairs, with keys in nibble-list format.
// ///     level :
// ///         Current trie level.
// /// 
// ///     Returns
// ///     -------
// ///     node : `ethereum.base_types.Bytes`
// ///         Root node of `obj`.
// ///     
// pub fn patricialize(obj: Mapping[Bytes][Bytes], level: Uint) -> Result<Optional[InternalNode], EthereumException> {
//     if len(obj)? == 0 {
//         return Ok(());
//     }
//     arbitrary_key = next(iter(obj)?)?;
//     if len(obj)? == 1 {
//         leaf = LeafNode(arbitrary_key[level..], obj[arbitrary_key])?;
//         return Ok(leaf);
//     }
//     substring = arbitrary_key[level..];
//     prefix_length = len(substring)?;
//     for key in obj {
//         prefix_length = min(prefix_length, common_prefix_length(substring, key[level..])?)?;
//         if prefix_length == 0 {
//             break;
//         }
//     }
//     if prefix_length > 0 {
//         prefix = arbitrary_key[level..level + prefix_length];
//         return Ok(ExtensionNode(prefix, encode_internal_node(patricialize(obj, level + prefix_length)?)?)?);
//     }
//     // TypedAssignment unsupported
//     for _ in range(16)? {
//         branches.append(/* DictLiteral unsupported */)?;
//     }
//     value = [];
//     for key in obj {
//         if len(key)? == level {
//             if isinstance(obj[key], (Account, Receipt, Uint))? {
//                 return Err(Error::AssertionError);
//             }
//             value = obj[key];
//         } else {
//             branches[key[level]][key] = obj[key];
//         }
//     }
//     return Ok(BranchNode(/* ListComp unsupported */, value)?);
// }
