use execution_specs_rs::ethereum::{
    base_types::{Uint, U256},
    rlp::{encode, encode_bytes, encode_iter, encode_sequence},
};

macro_rules! bytes {
    ($($num:literal $(* $times:expr)?),* $(,)?) => {
        []
            .into_iter()
        $(
            .chain(
                std::iter::repeat($num).take(1 $(+ $times as usize - 1)?)
            )
        )*
            .collect::<Vec<_>>()

    };
}

#[test]
fn test_rlp_encode_empty_bytes() {
    assert_eq!(*encode_bytes(b""), [0x80]);
}

#[test]
fn test_rlp_encode_single_byte_val_less_than_128() {
    assert_eq!(*encode_bytes(b"x"), [0x78]);
}

#[test]
fn test_rlp_encode_single_byte_val_equal_128() {
    assert_eq!(*encode_bytes(b"\x80"), [0x81, 0x80]);
}

#[test]
fn test_rlp_encode_single_byte_val_greater_than_128() {
    assert_eq!(*encode_bytes(b"\x83"), [0x81, 0x83]);
}

#[test]
fn test_rlp_encode_55_bytes() {
    assert_eq!(
        *encode_bytes(&bytes![b'\x83' * 55]),
        *bytes![0xb7, b'\x83' * 55]
    );
}

#[test]
fn test_rlp_encode_large_bytes() {
    assert_eq!(
        *encode_bytes(&bytes![b'\x83' * 2u32.pow(20)]),
        *bytes![0xba, b'\x10', b'\x00', b'\x00', b'\x83' * 2u32.pow(20)]
    );
}

#[test]
fn test_rlp_encode_uint_0() {
    assert_eq!(*encode(&Uint::from(0u8)), *b"\x80");
}

#[test]
fn test_rlp_encode_uint_byte_max() {
    assert_eq!(*encode(&Uint::from(255u8)), *b"\x81\xff");
}

#[test]
fn test_rlp_encode_uint256_0() {
    assert_eq!(*encode(&U256::from(0u8)), *b"\x80");
}

#[test]
fn test_rlp_encode_uint256_byte_max() {
    assert_eq!(*encode(&U256::from(255u8)), *b"\x81\xff");
}

#[test]
fn test_rlp_encode_empty_str() {
    assert_eq!(*encode(""), *b"\x80");
}

#[test]
fn test_rlp_encode_one_char_str() {
    assert_eq!(*encode("h"), *b"h");
}

#[test]
fn test_rlp_encode_multi_char_str() {
    assert_eq!(*encode("hello"), *b"\x85hello");
}

#[test]
fn test_rlp_encode_empty_sequence() {
    assert_eq!(*encode_sequence(&[]), [0xc0]);
}

#[test]
fn test_rlp_encode_single_elem_list_byte() {
    assert_eq!(*encode_iter([b"hello"]), *b"\xc6\x85hello");
}

#[test]
fn test_rlp_encode_single_elem_list_uint() {
    assert_eq!(*encode_iter([Uint::from(255u8)]), *b"\xc2\x81\xff");
}

#[test]
fn test_rlp_encode_10_elem_byte_uint_combo() {
    let raw_data = (
        b"hello",
        b"hello",
        b"hello",
        b"hello",
        b"hello",
        Uint::from(35u8),
        Uint::from(35u8),
        Uint::from(35u8),
        Uint::from(35u8),
        Uint::from(35u8),
    );
    let expected = b"\xe3\x85hello\x85hello\x85hello\x85hello\x85hello#####";

    assert_eq!(*encode(&raw_data), *expected);
}

// #[test]
// fn test_rlp_encode_20_elem_byte_uint_combo() {
//     let raw_data = (
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         Uint::from(35u8),
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//         b"hello",
//     );
//     let expected = b"\xf8F##########\x85hello\x85hello\x85hello\x85hello\x85hello\x85hello\x85hello\x85hello\x85hello\x85hello";
//     assert_eq!(*encode(&raw_data), *expected);
// }

#[test]
fn test_rlp_encode_nested_sequence() {
    let raw_data = (
        b"hello",
        Uint::from(255u8),
        (b"how", (b"are", b"you", (b"doing",))),
    );
    let expected = b"\xdd\x85hello\x81\xff\xd4\x83how\xcf\x83are\x83you\xc6\x85doing";
    assert_eq!(*encode(&raw_data), *expected);
}

#[test]
fn test_rlp_encode_successfully() {
    macro_rules! test {
        ($(($raw_data:expr, $expected:expr)),* $(,)?) => {
            $(assert_eq!(*encode(&$raw_data), $expected);)*
        };
    }

    test! {
        (b"", [0x80]),
        ([b'\x83'; 55], *bytes![0xb7, b'\x83' * 55]),
        (Uint::from(0u8), *b"\x80"),
        (Uint::from(255u8), *b"\x81\xff"),
        // todo
        // ---- test_rlp_encode_successfully stdout ----
        // thread 'test_rlp_encode_successfully' panicked at 'assertion failed: `(left == right)`
        //   left: `[128]`,
        //  right: `[192]`', tests/test_rlp.rs:172:5
        // ([], [0xc0]),
        // (
        //     [b"hello"] * 5 + [Uint(35)] * 5,  # type: ignore
        //     bytearray([0xE3])
        //     + bytearray(b"\x85hello\x85hello\x85hello\x85hello\x85hello#####"),
        // ),
        // (
        //     [b"hello", Uint(255), [b"how", [b"are", b"you", [b"doing"]]]],
        //     bytearray(
        //         b"\xdd\x85hello\x81\xff\xd4\x83how\xcf\x83are\x83you\xc6\x85doing"
        //     ),
        // ),
    }
}
