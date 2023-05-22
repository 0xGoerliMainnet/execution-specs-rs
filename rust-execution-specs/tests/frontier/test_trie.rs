
use execution_specs_rs::ethereum::{exceptions::EthereumException, frontier::{trie::{Trie, trie_set, root, dummy_root}}, base_types::Bytes, utils::hexadecimal::{has_hex_prefix, hex_to_bytes, hex}};

pub fn to_bytes(data: &serde_json::Value) -> Result<Bytes, EthereumException> {
    if data.is_null() {
        Ok(Bytes::default())
    } else if data.is_string() && has_hex_prefix(data.as_str().unwrap()) {
        Ok(hex_to_bytes(data.as_str().unwrap())?)
    } else if data.is_string() {
        Ok(Bytes::from(data.as_str().unwrap().as_ref()))
    } else {
        todo!();
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
    for (_name, test) in tests.as_object().unwrap() {
        let mut st = Trie::<Bytes, Bytes>::new(false, Bytes::default());
        for t in test["in"].as_array().unwrap() {
            let t = t.as_array().unwrap();
            trie_set(&mut st, to_bytes(&t[0])?, to_bytes(&t[1])?);
        }
        // println!("{:#?}", st);
        let result = root(&st, dummy_root);
        let expected = test["root"].as_str().unwrap();
        assert_eq!(hex(&result), expected);
    }
    Ok(())
}

#[test]
fn quick_test() {
    let mut t = Trie::<String, String>::new(false, "".into());
    trie_set(&mut t, "abc".to_string(),"abc".to_string());
    trie_set(&mut t, "abcd".to_string(),"abcd".to_string());
    // trie_set(&mut t, "hello".to_string(),"abcdefghijklmnopqrstuvwxyz".to_string());
    // trie_set(&mut t, "".to_string(),"abcdefghijklmnopqrstuvwxyz".to_string());
    let root = root(&t, dummy_root);
    println!("root={:?}", hex(&root));
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
    let path = format!("{ethereum_tests_path}/TrieTests/{path}");
    let json = std::fs::read_to_string(&path)
        .map_err(|_| EthereumException::FileNotFound(path))?;
    let value : serde_json::Value = serde_json::from_str(&json).unwrap();
    Ok(value)
}


