
use execution_specs_rs::ethereum::{exceptions::EthereumException, frontier::{trie::{Trie, trie_set, root, dummy_root}}, base_types::Bytes, utils::hexadecimal::{has_hex_prefix, hex_to_bytes, hex}};

fn string_to_bytes(data: &str) -> Result<Bytes, EthereumException> {
    if has_hex_prefix(data) {
        hex_to_bytes(data)
    } else {
        Ok(Bytes::from(data.as_bytes()))
    }
}

fn to_bytes(data: &serde_json::Value) -> Result<Bytes, EthereumException> {
    if data.is_null() {
        Ok(Bytes::default())
    } else if data.is_string() {
        string_to_bytes(data.as_str().unwrap())
    } else {
        todo!();
    }
}

#[test]
pub fn test_trie_secure_hex() -> Result<(), EthereumException> {
    let tests = load_tests("hex_encoded_securetrie_test.json")?;
    for (name, test) in tests.as_object().unwrap() {
        println!("name={}", name);
        let mut st = Trie::<Bytes, Bytes>::new(true, Bytes::default());
        for (k, v) in test["in"].as_object().unwrap() {
            trie_set(&mut st, string_to_bytes(&k)?, to_bytes(&v)?);
        }
        let result = root(&st, dummy_root);
        let expected = test["root"].as_str().unwrap();
        assert_eq!(hex(&result), expected);
    }
    Ok(())
}

#[test]
pub fn test_trie_secure() -> Result<(), EthereumException> {
    let tests = load_tests("trietest_secureTrie.json")?;
    for (name, test) in tests.as_object().unwrap() {
        println!("name={}", name);
        let mut st = Trie::<Bytes, Bytes>::new(true, Bytes::default());
        for t in test["in"].as_array().unwrap() {
            let t = t.as_array().unwrap();
            trie_set(&mut st, to_bytes(&t[0])?, to_bytes(&t[1])?);
        }
        let result = root(&st, dummy_root);
        let expected = test["root"].as_str().unwrap();
        assert_eq!(hex(&result), expected);
    }
    Ok(())
}

#[test]
pub fn test_trie_secure_any_order() -> Result<(), EthereumException> {
    let tests = load_tests("trieanyorder_secureTrie.json")?;
    for (name, test) in tests.as_object().unwrap() {
        println!("name={}", name);
        let mut st = Trie::<Bytes, Bytes>::new(true, Bytes::default());
        for (k, v) in test["in"].as_object().unwrap() {
            trie_set(&mut st, string_to_bytes(&k)?, to_bytes(&v)?);
        }
        let result = root(&st, dummy_root);
        let expected = test["root"].as_str().unwrap();
        assert_eq!(hex(&result), expected);
    }
    Ok(())
}

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
pub fn test_trie_any_order() -> Result<(), EthereumException> {
    let tests = load_tests("trieanyorder.json")?;
    for (name, test) in tests.as_object().unwrap() {
        println!("name={}", name);
        let mut st = Trie::<Bytes, Bytes>::new(false, Bytes::default());
        for (k, v) in test["in"].as_object().unwrap() {
            trie_set(&mut st, string_to_bytes(&k)?, to_bytes(&v)?);
        }
        let result = root(&st, dummy_root);
        let expected = test["root"].as_str().unwrap();
        assert_eq!(hex(&result), expected);
    }
    Ok(())
}


pub fn load_tests(path: &str) -> Result<serde_json::Value, EthereumException> {
    let ethereum_tests_path = "../tests/";
    let path = format!("{ethereum_tests_path}/TrieTests/{path}");
    let json = std::fs::read_to_string(&path)
        .map_err(|_| EthereumException::FileNotFound(path))?;
    let value : serde_json::Value = serde_json::from_str(&json).unwrap();
    Ok(value)
}


