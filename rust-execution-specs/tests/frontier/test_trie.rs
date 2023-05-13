
use rust_execution_specs::ethereum::{exceptions::EthereumException, frontier::trie::{Trie, trie_set}, base_types::Bytes, utils::hexadecimal::{has_hex_prefix, hex_to_bytes, remove_hex_prefix}};

pub fn to_bytes(data: &serde_json::Value) -> Result<Bytes, EthereumException> {
    if data.is_null() {
        Ok(Bytes::default())
    } else if has_hex_prefix(data.as_str().unwrap()) {
        Ok(hex_to_bytes(data.as_str().unwrap())?)
    } else {
        todo!();
        // Ok(data.encode())
    }
}

#[test]
pub fn test_trie_secure_hex() -> Result<(), EthereumException> {
    // let tests = load_tests("hex_encoded_securetrie_test.json")?;
    // for (name, test) in tests.as_object().unwrap() {
    //     println!("name={}", name);
    //     let mut st = Trie::<Vec<u8>, Vec<u8>>::new(true);
    //     for (k, v) in test["in"].as_object().unwrap() {
    //         // trie_set(st, hex_to_bytes(&k)?, to_bytes(&v)?)?;
    //         println!("{:?} {:?}", hex_to_bytes(&k)?, to_bytes(&v)?);
    //     }
    //     // let result = root(st)?;
    //     let expected = remove_hex_prefix(test["root"].as_str().unwrap());
    //     // assert!(result.hex()? == expected, "test {name} failed");
    //     println!("expected={:?}", expected);
    // }
    Ok(())
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


#[test]
pub fn test_trie() -> Result<(), EthereumException> {
    let tests = load_tests("trietest.json")?;
    for (name, test) in tests.as_object().unwrap() {
        let mut st = Trie::<String, String>::new(false, String::default());
        for t in test["in"].as_array().unwrap() {
            let t = t.as_array().unwrap();
            let key = t[0].as_str().unwrap().to_owned();
            let value = (if t[1].is_null() { "" } else { t[1].as_str().unwrap() }).to_owned();
            trie_set(&mut st, key, value);
        }
        println!("{:#?}", st);
        // result = root(st)?;
        // expected = remove_hex_prefix(test.get("root")?)?;
        // assert!(result.hex()? == expected, "test {name} failed");
    }
    Ok(())
}


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


pub fn load_tests(path: &str) -> Result<serde_json::Value, EthereumException> {
    let ethereum_tests_path = "../tests/";
    println!("{:?}", std::fs::canonicalize(&"./"));
    let path = format!("{ethereum_tests_path}/TrieTests/{path}");
    let json = std::fs::read_to_string(&path)
        .map_err(|_| EthereumException::FileNotFound(path))?;
    let value : serde_json::Value = serde_json::from_str(&json).unwrap();
    Ok(value)
}


