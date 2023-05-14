use std::{dbg, assert_eq};

use hex_literal::hex;
use num_bigint::BigUint;
use num_traits::cast::{FromPrimitive, self};
use execution_specs_rs::ethereum::base_types::{U256, Bytes};
use execution_specs_rs::ethereum::exceptions::EthereumException;
use execution_specs_rs::ethereum::genesis::{get_genesis_configuration, GenesisConfiguration};
use execution_specs_rs::ethereum::rlp;
use execution_specs_rs::ethereum::utils::hexadecimal::hex_to_bytes;

fn mainnet_genesis_configuration() -> GenesisConfiguration {
    get_genesis_configuration("mainnet.json").unwrap()
}

// NOTE: function has decorators
pub fn mainnet_alloc_rlp_encoding() -> Result<Bytes, EthereumException> {
    let path = format!("./assets/mainnet_genesis_alloc_rlp.hex");
    let rlp_encoding_hex = std::fs::read_to_string(&path).map_err(|_| EthereumException::FileNotFound(path))?;

    return Ok(hex_to_bytes(&rlp_encoding_hex)?);
}

#[test]
pub fn test_mainnet_alloc_rlp_encoding() {
    let mainnet_alloc_rlp_encoding = mainnet_alloc_rlp_encoding().unwrap();
    let mainnet_genesis_configuration: GenesisConfiguration = mainnet_genesis_configuration();

    let mut alocs: Vec<_> = Vec::with_capacity(mainnet_genesis_configuration.initial_balances.len());
    for (address, balance) in mainnet_genesis_configuration.initial_balances {
        alocs.push([U256::from_bytes_be(address.as_slice()), balance]);
    }
    alocs.sort_by(|a, b| a[0].cmp(&b[0]));

    let alloc_rlp_encoding = rlp::encode(alocs.as_slice());
    assert_eq!(alloc_rlp_encoding, mainnet_alloc_rlp_encoding);
}

// #[test]
// pub fn test_rlp_decode_mainnet_alloc_rlp_encoding()  {
//     decoded_alloc = cast(List[List[Bytes]], rlp.decode(mainnet_alloc_rlp_encoding)?)?;
//     obtained_alloc = /* DictComp unsupported */;
//     assert!(obtained_alloc == MAINNET_GENESIS_CONFIGURATION.initial_balances);
// }

#[test]
pub fn test_mainnet_genesis_config() {
    let mainnet_genesis_configuration: GenesisConfiguration = mainnet_genesis_configuration();
    assert_eq!(
        mainnet_genesis_configuration.difficulty,
        BigUint::from_u64(0x400000000).unwrap()
    );
    assert_eq!(
        mainnet_genesis_configuration.extra_data.as_ref(),
        hex!("11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa")
    );
    assert_eq!(
        mainnet_genesis_configuration.gas_limit,
        BigUint::from_u64(5000).unwrap()
    );
    assert_eq!(
        mainnet_genesis_configuration.nonce,
        [0, 0, 0, 0, 0, 0, 0, 66]
    );
    assert_eq!(
        mainnet_genesis_configuration.timestamp,
        BigUint::from_u64(0).unwrap()
    );
}
