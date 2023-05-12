// use ::typing::{Any};
// use ::ethereum::frontier::fork_types::{Bytes};
// use ::ethereum::frontier::trie::{Trie, root, trie_set};
// use ::ethereum::utils::hexadecimal::{has_hex_prefix, hex_to_bytes, remove_hex_prefix};
// use ::tests::helpers::{TEST_FIXTURES};
// ETHEREUM_TESTS_PATH = TEST_FIXTURES["ethereum_tests"]["fixture_path"];

pub fn to_bytes(data: &str) -> Result<Bytes, EthereumException> {
    if data.is_empty() {
        Ok(Bytes::default())
    } else if has_hex_prefix(data) {
        Ok(hex_to_bytes(data)?)
    } else {
        Ok(data.encode())
    }
}

pub fn test_trie_secure_hex() -> Result<(), EthereumException> {
    tests = load_tests("hex_encoded_securetrie_test.json")?;
    for (name, test) in tests.items()? {
        // TypedAssignment unsupported
        for (k, v) in test.get("in")?.items()? {
            trie_set(st, to_bytes(k)?, to_bytes(v)?)?;
        }
        result = root(st)?;
        expected = remove_hex_prefix(test.get("root")?)?;
        assert!(result.hex()? == expected, "test {name} failed");
    }
}


// pub fn test_trie_secure() -> Result<(), EthereumException> {
//     tests = load_tests("trietest_secureTrie.json")?;
//     for (name, test) in tests.items()? {
//         // TypedAssignment unsupported
//         for t in test.get("in")? {
//             trie_set(st, to_bytes(t[0])?, to_bytes(t[1])?)?;
//         }
//         result = root(st)?;
//         expected = remove_hex_prefix(test.get("root")?)?;
//         assert!(result.hex()? == expected, "test {name} failed");
//     }
// }


// pub fn test_trie_secure_any_order() -> Result<(), EthereumException> {
//     tests = load_tests("trieanyorder_secureTrie.json")?;
//     for (name, test) in tests.items()? {
//         // TypedAssignment unsupported
//         for (k, v) in test.get("in")?.items()? {
//             trie_set(st, to_bytes(k)?, to_bytes(v)?)?;
//         }
//         result = root(st)?;
//         expected = remove_hex_prefix(test.get("root")?)?;
//         assert!(result.hex()? == expected, "test {name} failed");
//     }
// }


// pub fn test_trie() -> Result<(), EthereumException> {
//     tests = load_tests("trietest.json")?;
//     for (name, test) in tests.items()? {
//         // TypedAssignment unsupported
//         for t in test.get("in")? {
//             trie_set(st, to_bytes(t[0])?, to_bytes(t[1])?)?;
//         }
//         result = root(st)?;
//         expected = remove_hex_prefix(test.get("root")?)?;
//         assert!(result.hex()? == expected, "test {name} failed");
//     }
// }


// pub fn test_trie_any_order() -> Result<(), EthereumException> {
//     tests = load_tests("trieanyorder.json")?;
//     for (name, test) in tests.items()? {
//         // TypedAssignment unsupported
//         for (k, v) in test.get("in")?.items()? {
//             trie_set(st, to_bytes(k)?, to_bytes(v)?)?;
//         }
//         result = root(st)?;
//         expected = remove_hex_prefix(test.get("root")?)?;
//         assert!(result.hex()? == expected, "test {name} failed");
//     }
// }


// pub fn load_tests(path: str) -> Result<Any, EthereumException> {
//     // with open("{ETHEREUM_TESTS_PATH}/TrieTests/" + path)?
//         tests = json.load(f)?;
//     return Ok(tests);
// }


