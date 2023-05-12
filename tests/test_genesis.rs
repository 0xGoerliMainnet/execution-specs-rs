
use hex_literal::hex;
use num_bigint::BigUint;
use rust_execution_specs::ethereum::genesis::{get_genesis_configuration, GenesisConfiguration};
use num_traits::cast::FromPrimitive;

fn mainnet_genesis_configuration() -> GenesisConfiguration {
    get_genesis_configuration("mainnet.json").unwrap()
}

// // NOTE: function has decorators
// pub fn mainnet_alloc_rlp_encoding() -> Result<bytes, Error> {
//     rlp_encoding_hex = cast(bytes, pkgutil.get_data("ethereum", "assets/mainnet_genesis_alloc_rlp.hex")?)?.decode()?;
//     return Ok(bytes.fromhex(rlp_encoding_hex)?);
// }

// pub fn test_mainnet_alloc_rlp_encoding(mainnet_alloc_rlp_encoding: bytes) -> Result<(), Error> {
//     alloc_rlp_encoding = rlp.encode(/* ListComp unsupported */)?;
//     assert!(alloc_rlp_encoding == mainnet_alloc_rlp_encoding);
// }


// pub fn test_rlp_decode_mainnet_alloc_rlp_encoding(mainnet_alloc_rlp_encoding: bytes) -> Result<(), Error> {
//     decoded_alloc = cast(List[List[Bytes]], rlp.decode(mainnet_alloc_rlp_encoding)?)?;
//     obtained_alloc = /* DictComp unsupported */;
//     assert!(obtained_alloc == MAINNET_GENESIS_CONFIGURATION.initial_balances);
// }


#[test]
pub fn test_mainnet_genesis_config() {
    let mainnet_genesis_configuration = mainnet_genesis_configuration();
    assert_eq!(mainnet_genesis_configuration.difficulty, BigUint::from_u64(0x400000000).unwrap());
    assert_eq!(mainnet_genesis_configuration.extra_data.as_ref(), hex!("11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa"));
    assert_eq!(mainnet_genesis_configuration.gas_limit, BigUint::from_u64(5000).unwrap());
    assert_eq!(mainnet_genesis_configuration.nonce, [0, 0, 0, 0, 0, 0, 0, 66]);
    assert_eq!(mainnet_genesis_configuration.timestamp, BigUint::from_u64(0).unwrap());

    // todo: allocs.    
}


