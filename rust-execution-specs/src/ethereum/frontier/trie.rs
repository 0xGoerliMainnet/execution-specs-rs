//! 
//! # State Trie
//! 
//! ## Introduction
//! 
//! The state trie is the structure responsible for storing
//! `.fork_types.Account` objects.
//! 
#![allow(dead_code)]

use std::collections::HashMap;

use crate::ethereum::{rlp::{EncodeRlp}, base_types::{Bytes, U256, Bytes32}, exceptions::EthereumException};

use super::fork_types::{keccak256, Account, Address, Root};

pub trait Key : Eq + std::hash::Hash + AsRef<[u8]> + Clone {}

#[allow(non_snake_case)]
pub fn EMPTY_TRIE_ROOT() -> Root {
    let value = hex_literal::hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
    Root::from(value)
}

pub trait Node: PartialEq + std::fmt::Debug + Clone {
    fn encode<F : Fn(&Address) -> Root>(&self, f: F) -> Bytes;
}

impl Node for String {
    fn encode<F : Fn(&Address) -> Root>(&self, _f: F) -> Bytes {
        self.as_bytes().into()
    }
}

impl Node for Bytes {
    fn encode<F : Fn(&Address) -> Root>(&self, _f: F) -> Bytes {
        self.clone()
    }
}

impl Node for () {
    fn encode<F : Fn(&Address) -> Root>(&self, _f: F) -> Bytes {
        Bytes::default()
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

impl Key for Bytes {}
impl Key for String {}
impl Key for Address {}
impl Key for Bytes32 {}

/// The possible return values of the function [patricialize].
#[derive(Debug)]
pub enum InternalNode {
    /// The leaf of the Trie containing the value.
    LeafNode{
        rest_of_key: Bytes,
        value: Bytes,
    },
    /// A node representing a group of keys with a common prefix.
    ExtensionNode{
        key_segment: Bytes,
        subnode: Encodable,
    },
    /// A node representing a group of keys without a common prefix.
    BranchNode{
        subnodes: Vec<Encodable>,
        value: Bytes,
    },
    None,
}

#[derive(Debug)]
pub enum Encodable {
    Bytes(Bytes),
    RLP(Box<dyn EncodeRlp>),
    Root(Root),
}

impl EncodeRlp for Encodable {
    fn encode(&self) -> Bytes {
        match self {
            Encodable::Bytes(bytes) => bytes.as_ref().encode(),
            Encodable::RLP(rlp) => rlp.encode(),
            Encodable::Root(root) => root.encode(),
        }
    }
}

/// 
/// Encodes a Merkle Trie node into its RLP form. The RLP will then be
/// serialized into a `Bytes` and hashed unless it is less that 32 bytes
/// when serialized.
/// 
/// This function also accepts `None`, representing the absence of a node,
/// which is encoded to `b""`.
/// 
/// ## Parameters
/// node : Optional[InternalNode]
///     The node to encode.
/// 
/// ## Returns
/// encoded : `rlp.RLP`
///     The node encoded as RLP.
/// 
/// ## Rust note:
/// 
/// This code follows the Python model as close as is possbile.
/// The use of Dyn is significantly suboptimal, but the code is illustrative only.
/// 
pub fn encode_internal_node(node: InternalNode) -> Encodable {
    let unencoded : Box<dyn EncodeRlp> = match node {
        InternalNode::LeafNode{rest_of_key, value} => {
            Box::new((
                nibble_list_to_compact(&rest_of_key, true),
                value
            ))
        },
        InternalNode::ExtensionNode{key_segment, subnode} => {
            Box::new((
                nibble_list_to_compact(&key_segment, false),
                subnode
            ))
        },
        InternalNode::BranchNode{mut subnodes, value} => {
            subnodes.push(Encodable::Bytes(value));
            Box::new(
                subnodes
            )
        },
        InternalNode::None => {
            Box::new("")
        },
    };

    let encoded = unencoded.encode();
    if encoded.len() < 32 {
        Encodable::RLP(unencoded)
    } else {
        Encodable::Root(keccak256(&encoded))
    }
}


/// 
/// Encode a Node for storage in the Merkle Trie.
/// 
/// Currently mostly an unimplemented stub.
/// 
pub fn encode_node<N : Node, F : Fn(&Address) -> Root>(node: &N, f: F) -> Bytes {
    node.encode(f)
}

/// 
/// The Merkle Trie.
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
    /// Create a new Trie.
    /// A secured trie hashes its keys.
    /// The default value is not included in the root.
    pub fn new(secured: bool, default: V) -> Self {
        Self {
            secured,
            default,
            data: HashMap::new(),
        }
    }
}


/// 
/// Create a copy of `trie`. Since only frozen objects may be stored in tries,
/// the contents are reused.
/// 
/// Parameters
/// ----------
/// trie: `Trie`
///     Trie to copy.
/// 
/// Returns
/// -------
/// new_trie : `Trie[K, V]`
///     A copy of the trie.
/// 
pub fn copy_trie<K, V>(trie: Trie<K, V>) -> Trie<K, V>
where
    K: Key, V: Node,
{
    trie.clone()
}


/// 
/// Stores an item in a Merkle Trie.
/// 
/// This method deletes the key if `value == trie.default`, because the Merkle
/// Trie represents the default value by omitting it from the trie.
/// 
/// Parameters
/// ----------
/// trie: `Trie`
///     Trie to store in.
/// key : `Bytes`
///     Key to lookup.
/// value : `V`
///     Node to insert at `key`.
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
/// Gets an item from the Merkle Trie.
/// 
/// This method returns `trie.default` if the key is missing.
/// 
/// Parameters
/// ----------
/// trie:
///     Trie to lookup in.
/// key :
///     Key to lookup.
/// 
/// Returns
/// -------
/// node : `V`
///     Node at `key` in the trie.
/// 
pub fn trie_get<'t, K, V>(trie: &'t Trie<K, V>, key: &K) -> V
where
    K: Key, V: Node,
{
    trie.data.get(key).cloned().unwrap_or_else(|| trie.default.clone())
}


/// 
/// Find the longest common prefix of two sequences.
/// 
pub fn common_prefix_length(a: &[u8], b: &[u8]) -> usize {
    for i in 0..a.len() {
        if i >= b.len() || a[i] != b[i] {
            return i;
        }
    }
    a.len()
}


/// 
/// Compresses nibble-list into a standard byte array with a flag.
/// 
/// A nibble-list is a list of byte values no greater than `15`. The flag is
/// encoded in high nibble of the highest byte. The flag nibble can be broken
/// down into two two-bit flags.
/// 
/// Highest nibble::
/// 
///     +---+---+----------+--------+
///     | _ | _ | is_leaf | parity |
///     +---+---+----------+--------+
///       3   2      1         0
/// 
/// 
/// The lowest bit of the nibble encodes the parity of the length of the
/// remaining nibbles -- `0` when even and `1` when odd. The second lowest bit
/// is used to distinguish leaf and extension nodes. The other two bits are not
/// used.
/// 
/// Parameters
/// ----------
/// x :
///     Array of nibbles.
/// is_leaf :
///     True if this is part of a leaf node, or false if it is an extension
///     node.
/// 
/// Returns
/// -------
/// compressed : `bytearray`
///     Compact byte array.
/// 
pub fn nibble_list_to_compact(x: &[u8], is_leaf: bool) -> Bytes {
    let mut compact = vec![];
    if x.len() % 2 == 0 {
        compact.push(16 * 2 * is_leaf as u8);
        for i in (0..x.len()).step_by(2) {
            compact.push(16 * x[i] + x[i + 1]);
        }
    } else {
        compact.push(16 * (2 * is_leaf as u8 + 1) + x[0]);
        for i in (1..x.len()).step_by(2) {
            compact.push(16 * x[i] + x[i + 1]);
        }
    }
    Box::from(compact)
}


/// 
/// Converts a `Bytes` into to a sequence of nibbles (bytes with value < 16).
/// 
/// Parameters
/// ----------
/// bytes_:
///     The `Bytes` to convert.
/// 
/// Returns
/// -------
/// nibble_list : `Bytes`
///     The `Bytes` in nibble-list format.
/// 
pub fn bytes_to_nibble_list(bytes: &[u8]) -> Bytes {
    let mut res = vec![0; bytes.len()*2];
    for i in 0..bytes.len() {
        res[i*2 + 0] = bytes[i] >> 4;
        res[i*2 + 1] = bytes[i] & 15;
    }
    Bytes::from(res)
}


/// 
/// Prepares the trie for root calculation. Removes values that are empty,
/// hashes the keys (if `secured == True`) and encodes all the nodes.
/// 
/// Parameters
/// ----------
/// trie :
///     The `Trie` to prepare.
/// get_storage_root :
///     Function to get the storage root of an account. Needed to encode
///     `Account` objects.
/// 
/// Returns
/// -------
/// out : `Mapping[ethereum.base_types.Bytes, Node]`
///     Object with keys mapped to nibble-byte form.
/// 
fn _prepare_trie<K, V, F>(trie: &Trie<K, V>, f: F) -> Result<Vec<(Bytes, Bytes)>, EthereumException>
where
    K: Key, V: Node,
    F : Fn(&Address) -> Root + Clone,
{
    let mut res = vec![];
    for (preimage, value) in &trie.data {
        let preimage = preimage.as_ref();
        let f = f.clone();
        let encoded_value = encode_node(value, f);
        assert!(!encoded_value.is_empty());
        if trie.secured {
            res.push((bytes_to_nibble_list(&keccak256(preimage)), encoded_value));
        } else {
            res.push((bytes_to_nibble_list(preimage), encoded_value));
        };
    }
    Ok(res)
}

/// 
/// Computes the root of a modified merkle patricia trie (MPT).
/// 
/// Parameters
/// ----------
/// trie :
///     `Trie` to get the root of.
/// get_storage_root :
///     Function to get the storage root of an account. Needed to encode
///     `Account` objects.
/// 
/// 
/// Returns
/// -------
/// root : `.fork_types.Root`
///     MPT root of the underlying key-value pairs.
/// 
pub fn root<K, V, F : Fn(&Address) -> Root + Clone>(trie: &Trie<K, V>, f: F) -> Root
where
    K: Key, V: Node,
{
    let obj = _prepare_trie(&trie, f).unwrap();
    match encode_internal_node(patricialize(obj, 0)) {
        Encodable::RLP(rlp) => {
            let encoded = rlp.encode();
            Root::from(keccak256(&encoded))
        }
        Encodable::Root(root) => root,
        Encodable::Bytes(_) => unreachable!(),
    }
}


/// 
/// Structural composition function.
/// 
/// Used to recursively patricialize and merkleize a dictionary. Includes
/// memoization of the tree structure and hashes.
/// 
/// Parameters
/// ----------
/// obj :
/// Underlying trie key-value pairs, with keys in nibble-list format.
/// level :
/// Current trie level.
/// 
/// Returns
/// -------
/// node : `ethereum.base_types.Bytes`
/// Root node of `obj`.
/// 
pub fn patricialize(mut obj: Vec<(Bytes, Bytes)>, level: usize) -> InternalNode {
    if obj.is_empty() {
        return InternalNode::None;
    }

    if obj.len() == 1 {
        let (arbitrary_key, value) = obj.pop().unwrap();
        return InternalNode::LeafNode{
            rest_of_key: Box::from(&arbitrary_key[level..]),
            value
        };
    }

    let arbitrary_key = &obj[0].0;
    let substring = &arbitrary_key[level..];
    let mut prefix_length = substring.len();
    for (key, _value) in obj.iter().skip(1) {
        prefix_length = <usize>::min(
            prefix_length, 
            common_prefix_length(substring, &key[level..])
        );
        if prefix_length == 0 {
            break;
        }
    }

    if prefix_length > 0 {
        let prefix : Bytes = arbitrary_key[level..level + prefix_length].into();
        let subnode = patricialize(obj, level + prefix_length);
        return InternalNode::ExtensionNode {
            key_segment: prefix,
            subnode: encode_internal_node(subnode),
        };
    }

    const EMPTY : Vec<(Bytes, Bytes)> = Vec::new();
    let mut branches = [EMPTY; 16];
    let mut value = Bytes::default();
    for (key, v) in obj {
        if key.len() == level {
            value = v;
        } else {
            branches[key[level] as usize].push((key, v));
        }
    }

    let subnodes = branches
        .into_iter()
        .map(|obj| encode_internal_node(patricialize(obj, level+1)))
        .collect::<Vec<_>>();

    InternalNode::BranchNode { subnodes, value }
}

/// A dummy root provider for when the value is not an address.
pub fn dummy_root(_: &Address) -> Root {
    Root::default()
}
