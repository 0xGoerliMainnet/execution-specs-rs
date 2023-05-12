// // NOTE: Import::Import unsupported
// // NOTE: Import::Import unsupported
// use ::typing::{List, Sequence, Tuple, Union, cast};
// // NOTE: Import::Import unsupported
// use ::ethereum::{rlp};
// use ::ethereum::exceptions::{RLPEncodingError};
// use ::ethereum::frontier::fork_types::{U256, Bytes, Uint};
// use ::ethereum::rlp::{RLP};
// use ::ethereum::utils::hexadecimal::{hex_to_bytes};
// use ::tests::helpers::{TEST_FIXTURES};
// ETHEREUM_TESTS_PATH = TEST_FIXTURES["ethereum_tests"]["fixture_path"];
// pub fn test_rlp_encode_empty_bytes() -> Result<(), Error> {
//     assert!(rlp.encode_bytes([])? == bytearray([128])?);
//     assert!(rlp.encode_bytes(bytearray()?)? == bytearray([128])?);
// }


// pub fn test_rlp_encode_single_byte_val_less_than_128() -> Result<(), Error> {
//     assert!(rlp.encode_bytes([120])? == bytearray([120])?);
//     assert!(rlp.encode_bytes(bytearray([120])?)? == bytearray([120])?);
// }


// pub fn test_rlp_encode_single_byte_val_equal_128() -> Result<(), Error> {
//     assert!(rlp.encode_bytes([128])? == [129, 128]);
//     assert!(rlp.encode_bytes(bytearray([128])?)? == [129, 128]);
// }


// pub fn test_rlp_encode_single_byte_val_greater_than_128() -> Result<(), Error> {
//     assert!(rlp.encode_bytes([131])? == bytearray([129, 131])?);
//     assert!(rlp.encode_bytes(bytearray([131])?)? == bytearray([129, 131])?);
// }


// pub fn test_rlp_encode_55_bytes() -> Result<(), Error> {
//     assert!(rlp.encode_bytes([131] * 55)? == bytearray([183])? + bytearray([131] * 55)?);
//     assert!(rlp.encode_bytes(bytearray([131])? * 55)? == bytearray([183])? + bytearray([131] * 55)?);
// }


// pub fn test_rlp_encode_large_bytes() -> Result<(), Error> {
//     assert!(rlp.encode_bytes([131] * (2).pow(20))? == bytearray([186])? + bytearray([16, 0, 0])? + bytearray([131] * (2).pow(20))?);
//     assert!(rlp.encode_bytes(bytearray([131])? * (2).pow(20))? == bytearray([186])? + bytearray([16, 0, 0])? + bytearray([131] * (2).pow(20))?);
// }


// pub fn test_rlp_encode_uint_0() -> Result<(), Error> {
//     assert!(rlp.encode(Uint(0)?)? == [128]);
// }


// pub fn test_rlp_encode_uint_byte_max() -> Result<(), Error> {
//     assert!(rlp.encode(Uint(255)?)? == [129, 255]);
// }


// pub fn test_rlp_encode_uint256_0() -> Result<(), Error> {
//     assert!(rlp.encode(U256(0)?)? == [128]);
// }


// pub fn test_rlp_encode_uint256_byte_max() -> Result<(), Error> {
//     assert!(rlp.encode(U256(255)?)? == [129, 255]);
// }


// pub fn test_rlp_encode_empty_str() -> Result<(), Error> {
//     assert!(rlp.encode("")? == [128]);
// }


// pub fn test_rlp_encode_one_char_str() -> Result<(), Error> {
//     assert!(rlp.encode("h")? == [104]);
// }


// pub fn test_rlp_encode_multi_char_str() -> Result<(), Error> {
//     assert!(rlp.encode("hello")? == [133, 104, 101, 108, 108, 111]);
// }


// pub fn test_rlp_encode_empty_sequence() -> Result<(), Error> {
//     assert!(rlp.encode_sequence([])? == bytearray([192])?);
// }


// pub fn test_rlp_encode_single_elem_list_byte() -> Result<(), Error> {
//     assert!(rlp.encode_sequence([[104, 101, 108, 108, 111]])? == bytearray([198])? + [133, 104, 101, 108, 108, 111]);
// }


// pub fn test_rlp_encode_single_elem_list_uint() -> Result<(), Error> {
//     assert!(rlp.encode_sequence([Uint(255)?])? == bytearray([194])? + [129, 255]);
// }


// pub fn test_rlp_encode_10_elem_byte_uint_combo() -> Result<(), Error> {
//     raw_data = [[104, 101, 108, 108, 111]] * 5 + [Uint(35)?] * 5;
//     expected = bytearray([227])? + [133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 35, 35, 35, 35, 35];
//     assert!(rlp.encode_sequence(raw_data)? == expected);
// }


// pub fn test_rlp_encode_20_elem_byte_uint_combo() -> Result<(), Error> {
//     raw_data = [Uint(35)?] * 10 + [[104, 101, 108, 108, 111]] * 10;
//     expected = bytearray([248])? + [70] + [35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111];
//     assert!(rlp.encode_sequence(raw_data)? == expected);
// }


// pub fn test_rlp_encode_nested_sequence() -> Result<(), Error> {
//     // TypedAssignment unsupported
//     // TypedAssignment unsupported
//     assert!(rlp.encode_sequence(nested_sequence)? == expected);
// }


// pub fn test_rlp_encode_successfully() -> Result<(), Error> {
//     test_cases = [([], bytearray([128])?), ([131] * 55, bytearray([183])? + bytearray([131] * 55)?), (Uint(0)?, [128]), (Uint(255)?, [129, 255]), ([], bytearray([192])?), ([[104, 101, 108, 108, 111]] * 5 + [Uint(35)?] * 5, bytearray([227])? + bytearray([133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 35, 35, 35, 35, 35])?), ([[104, 101, 108, 108, 111], Uint(255)?, [[104, 111, 119], [[97, 114, 101], [121, 111, 117], [[100, 111, 105, 110, 103]]]]], bytearray([221, 133, 104, 101, 108, 108, 111, 129, 255, 212, 131, 104, 111, 119, 207, 131, 97, 114, 101, 131, 121, 111, 117, 198, 133, 100, 111, 105, 110, 103])?)];
//     for (raw_data, expected_encoding) in test_cases {
//         assert!(rlp.encode(cast(RLP, raw_data)?)? == expected_encoding);
//     }
// }


// pub fn test_rlp_encode_fails() -> Result<(), Error> {
//     test_cases = [123, [[104, 101, 108, 108, 111], Uint(255)?, [[104, 111, 119], [[97, 114, 101], [[121, 111, 117], [123]]]]]];
//     for raw_data in test_cases {
//         // with pytest.raises(RLPEncodingError)?
//             rlp.encode(cast(RLP, raw_data)?)?;
//     }
// }


// pub fn test_rlp_decode_to_empty_bytes() -> Result<(), Error> {
//     assert!(rlp.decode_to_bytes(bytearray([128])?)? == []);
// }


// pub fn test_rlp_decode_to_single_byte_less_than_128() -> Result<(), Error> {
//     assert!(rlp.decode_to_bytes(bytearray([0])?)? == bytearray([0])?);
//     assert!(rlp.decode_to_bytes(bytearray([120])?)? == bytearray([120])?);
// }


// pub fn test_rlp_decode_to_single_byte_gte_128() -> Result<(), Error> {
//     assert!(rlp.decode_to_bytes(bytearray([129, 131])?)? == [131]);
//     assert!(rlp.decode_to_bytes([129, 128])? == [128]);
// }


// pub fn test_rlp_decode_to_55_bytes() -> Result<(), Error> {
//     encoding = bytearray([183])? + bytearray([131] * 55)?;
//     expected_raw_data = bytearray([131])? * 55;
//     assert!(rlp.decode_to_bytes(encoding)? == expected_raw_data);
// }


// pub fn test_rlp_decode_to_large_bytes() -> Result<(), Error> {
//     encoding = bytearray([186])? + [16, 0, 0] + [131] * (2).pow(20);
//     expected_raw_data = [131] * (2).pow(20);
//     assert!(rlp.decode_to_bytes(encoding)? == expected_raw_data);
// }


// pub fn test_rlp_decode_to_zero_uint() -> Result<(), Error> {
//     assert!(rlp.decode([128])? == Uint(0)?.to_be_bytes()?);
// }


// pub fn test_rlp_decode_to_255_uint() -> Result<(), Error> {
//     assert!(rlp.decode([129, 255])? == Uint(255)?.to_be_bytes()?);
// }


// pub fn test_rlp_decode_empty_str() -> Result<(), Error> {
//     assert!(rlp.decode([128])? == "".encode()?);
// }


// pub fn test_rlp_decode_one_char_str() -> Result<(), Error> {
//     assert!(rlp.decode([104])? == "h".encode()?);
// }


// pub fn test_rlp_decode_multi_char_str() -> Result<(), Error> {
//     assert!(rlp.decode([133, 104, 101, 108, 108, 111])? == "hello".encode()?);
// }


// pub fn test_rlp_decode_to_empty_sequence() -> Result<(), Error> {
//     assert!(rlp.decode_to_sequence(bytearray([192])?)? == []);
// }


// pub fn test_rlp_decode_to_1_elem_sequence_of_byte() -> Result<(), Error> {
//     assert!(rlp.decode_to_sequence(bytearray([198])? + [133, 104, 101, 108, 108, 111])? == [[104, 101, 108, 108, 111]]);
// }


// pub fn test_rlp_decode_to_1_elem_sequence_of_uint() -> Result<(), Error> {
//     assert!(rlp.decode_to_sequence(bytearray([194])? + [129, 255])? == [Uint(255)?.to_be_bytes()?]);
// }


// pub fn test_rlp_decode_to_10_elem_sequence_of_bytes_and_uints() -> Result<(), Error> {
//     encoded_data = bytearray([227])? + [133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 35, 35, 35, 35, 35];
//     expected_raw_data = [[104, 101, 108, 108, 111]] * 5 + [Uint(35)?.to_be_bytes()?] * 5;
//     assert!(rlp.decode_to_sequence(encoded_data)? == expected_raw_data);
// }


// pub fn test_rlp_decode_to_20_elem_sequence_of_bytes_and_uints() -> Result<(), Error> {
//     encoded_data = bytearray([248])? + [70] + [133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 133, 104, 101, 108, 108, 111, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35];
//     expected_raw_data = [[104, 101, 108, 108, 111]] * 10 + [Uint(35)?.to_be_bytes()?] * 10;
//     assert!(rlp.decode_to_sequence(encoded_data)? == expected_raw_data);
// }


// pub fn test_rlp_decode_to_nested_sequence() -> Result<(), Error> {
//     encoded_data = [223, 133, 104, 101, 108, 108, 111, 129, 255, 214, 131, 104, 111, 119, 209, 131, 97, 114, 101, 131, 121, 111, 117, 200, 133, 100, 111, 105, 110, 103, 193, 35];
//     expected_raw_data = [[104, 101, 108, 108, 111], Uint(255)?.to_be_bytes()?, [[104, 111, 119], [[97, 114, 101], [121, 111, 117], [[100, 111, 105, 110, 103], [Uint(35)?.to_be_bytes()?]]]]];
//     assert!(rlp.decode_to_sequence(encoded_data)? == expected_raw_data);
// }


// pub fn test_rlp_decode_successfully() -> Result<(), Error> {
//     test_cases = [(bytearray([128])?, bytearray()?), (bytearray([183])? + bytearray([131] * 55)?, bytearray([131])? * 55), (bytearray([192])?, []), ([219, 133, 104, 101, 108, 108, 111, 212, 131, 104, 111, 119, 207, 131, 97, 114, 101, 131, 121, 111, 117, 198, 133, 100, 111, 105, 110, 103], [[104, 101, 108, 108, 111], [[104, 111, 119], [[97, 114, 101], [121, 111, 117], [[100, 111, 105, 110, 103]]]]])];
//     for (encoding, expected_raw_data) in test_cases {
//         assert!(rlp.decode(encoding)? == expected_raw_data);
//     }
// }


// pub fn test_rlp_decode_failure_empty_bytes() -> Result<(), Error> {
//     // with pytest.raises(Exception)?
//         rlp.decode([])?;
// }


// pub fn test_roundtrip_encoding_and_decoding() -> Result<(), Error> {
//     test_cases = [[], [104], [104, 101, 108, 108, 111, 32, 104, 111, 119, 32, 97, 114, 101, 32, 121, 111, 117, 32, 100, 111, 105, 110, 103, 32, 116, 111, 100, 97, 121, 63], Uint(35)?.to_be_bytes()?, Uint(255)?.to_be_bytes()?, [], [[104, 101, 108, 108, 111], [[104, 111, 119], [[97, 114, 101], [121, 111, 117], [[100, 111, 105, 110, 103], [Uint(255)?.to_be_bytes()?]]]]], [[[104, 101, 108, 108, 111], [119, 111, 114, 108, 100]], [[104, 111, 119], [97, 114, 101]], [[121, 111, 117], [100, 111, 105, 110, 103]]]];
//     for raw_data in test_cases {
//         assert!(rlp.decode(rlp.encode(cast(RLP, raw_data)?)?)? == raw_data);
//     }
// }


// pub fn convert_to_rlp_native(obj: Union[str][int][Sequence[Union[str][int]]]) -> Result<RLP, Error> {
//     if isinstance(obj, str)? {
//         return Ok(bytes(obj, "utf-8")?);
//     } else if isinstance(obj, int)? {
//         return Ok(Uint(obj)?);
//     }
//     return Ok(/* ListComp unsupported */);
// }


// pub fn ethtest_fixtures_as_pytest_fixtures() -> Result<List[Tuple[RLP][Bytes]], Error> {
//     base_path = "{ETHEREUM_TESTS_PATH}/RLPTests/";
//     test_data = dict()?;
//     for test_file in test_files {
//         // with open(os.path.join(base_path, test_file)?, "r")?
//             test_data.update(json.load(fp)?)?;
//     }
//     pytest_fixtures = [];
//     for test_details in test_data.values()? {
//         if isinstance(test_details["in"], str)? && test_details["in"].startswith("#")? {
//             test_details["in"] = int(test_details["in"][1..])?;
//         }
//         pytest_fixtures.append((convert_to_rlp_native(test_details["in"])?, hex_to_bytes(test_details["out"])?))?;
//     }
//     return Ok(pytest_fixtures);
// }


// // NOTE: function has decorators
// pub fn test_ethtest_fixtures_for_rlp_encoding(raw_data: RLP, expected_encoded_data: Bytes) -> Result<(), Error> {
//     assert!(rlp.encode(raw_data)? == expected_encoded_data);
// }


// // NOTE: function has decorators
// pub fn test_ethtest_fixtures_for_successfull_rlp_decoding(raw_data: Bytes, encoded_data: Bytes) -> Result<(), Error> {
//     decoded_data = rlp.decode(encoded_data)?;
//     assert!(rlp.encode(decoded_data)? == encoded_data);
// }


// // NOTE: function has decorators
// pub fn test_ethtest_fixtures_for_fails_in_rlp_decoding(raw_data: Bytes, encoded_data: Bytes) -> Result<(), Error> {
//     // with pytest.raises(Exception)?
//         rlp.decode(encoded_data)?;
// }


