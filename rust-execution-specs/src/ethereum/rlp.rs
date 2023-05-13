use std::usize;

///
/// .. _rlp:
///
/// Recursive Length Prefix (RLP) Encoding
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///
/// .. contents:: Table of Contents
///     :backlinks: none
///     :local:
///
/// Introduction
/// ------------
///
/// Defines the serialization and deserialization format used throughout Ethereum.
///
// use ::__future__::{annotations};
// use ::dataclasses::{astuple, fields, is_dataclass};
// use ::typing::{Any, List, Sequence, Tuple, Type, TypeVar, Union};
// use ::ethereum::crypto::hash::{Hash32, keccak256};
// use ::ethereum::exceptions::{RLPDecodingError, RLPEncodingError};
// use ::ethereum::utils::ensure::{ensure};
use super::base_types::{strip_leading_zeros, Bytes, Uint, U32, U64};

pub trait EncodeRlp {
    // A somewhat suboptimal RLP encoder.
    fn encode(&self) -> Bytes;
}

///
///     Encodes `raw_data` into a sequence of bytes using RLP.
///
///     Parameters
///     ----------
///     raw_data :
///         A `Bytes`, `Uint`, `Uint256` or sequence of `RLP` encodable
///         objects.
///
///     Returns
///     -------
///     encoded : `ethereum.base_types.Bytes`
///         The RLP encoded bytes representing `raw_data`.
///
pub fn encode<R: ?Sized + EncodeRlp>(raw_data: &R) -> Bytes {
    raw_data.encode()
}

impl<T: ?Sized + EncodeRlp> EncodeRlp for &T {
    fn encode(&self) -> Bytes {
        T::encode(self)
    }
}

// if isinstance(raw_data, (bytearray, bytes))? {
//     return Ok(encode_bytes(raw_data)?);

impl EncodeRlp for Bytes {
    fn encode(&self) -> Bytes {
        encode_bytes(&self)
    }
}
impl<const N: usize> EncodeRlp for [u8; N] {
    fn encode(&self) -> Bytes {
        encode_bytes(self)
    }
}
impl EncodeRlp for [u8] {
    fn encode(&self) -> Bytes {
        encode_bytes(self)
    }
}

// } else if isinstance(raw_data, (Uint, FixedUInt))? {
//     return Ok(encode(raw_data.to_be_bytes()?)?);

impl EncodeRlp for Uint {
    fn encode(&self) -> Bytes {
        let bytes = self.to_bytes_be();
        encode_bytes(strip_leading_zeros(&bytes))
    }
}

impl EncodeRlp for U64 {
    fn encode(&self) -> Bytes {
        encode_bytes(strip_leading_zeros(&self.to_be_bytes()))
    }
}

impl EncodeRlp for U32 {
    fn encode(&self) -> Bytes {
        encode_bytes(strip_leading_zeros(&self.to_be_bytes()))
    }
}

// } else if isinstance(raw_data, str)? {
//     return Ok(encode_bytes(raw_data.encode()?)?);

impl EncodeRlp for String {
    fn encode(&self) -> Bytes {
        str::encode(self)
    }
}
impl EncodeRlp for str {
    fn encode(&self) -> Bytes {
        encode_bytes(&self.as_bytes())
    }
}

// } else if isinstance(raw_data, bool)? {
//     if raw_data {
//         return Ok(encode_bytes([1])?);
//     } else {
//         return Ok(encode_bytes([])?);
//     }

impl EncodeRlp for bool {
    fn encode(&self) -> Bytes {
        if *self {
            encode_bytes(&[1])
        } else {
            encode_bytes(&[])
        }
    }
}

// } else if isinstance(raw_data, Sequence)? {
//     return Ok(encode_sequence(raw_data)?);

impl<T: EncodeRlp> EncodeRlp for [T] {
    fn encode(&self) -> Bytes {
        let mut joined_encodings = vec![];
        for item in self {
            joined_encodings.extend(item.encode().iter().copied());
        }
        encode_sequence(&joined_encodings)
    }
}

impl<const N: usize, T: EncodeRlp> EncodeRlp for [T; N] {
    fn encode(&self) -> Bytes {
        let mut joined_encodings = vec![];
        for item in self {
            joined_encodings.extend(item.encode().iter().copied());
        }
        encode_sequence(&joined_encodings)
    }
}

// } else if is_dataclass(raw_data)? {
//     return Ok(encode(astuple(raw_data)?)?);
macro_rules! impl_tuples {
    (@__expand) => {};
    (@__expand $($t:ident)*) => {
        impl<$($t),*> EncodeRlp for ($($t,)*)
        where
            $($t: EncodeRlp),*
        {
            fn encode(&self) -> Bytes {
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                let mut joined_encodings = vec![];
                $(joined_encodings.extend($t.encode().iter().copied());)*
                encode_sequence(&joined_encodings)
            }
        }
    };
    (@__walk [] $($prev:tt)*) => {};
    (@__walk [$next:tt $($rest:tt)*] $($prev:tt)*) => {
        impl_tuples!(@__expand $($prev)* $next );
        impl_tuples!(@__walk [ $($rest)* ] $($prev)* $next );
    };
    ($($t:tt)*) => {
        impl_tuples!(@__walk [$($t)*]);
    };
}

impl_tuples!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19);

///
///     Encodes `raw_bytes`, a sequence of bytes, using RLP.
///
///     Parameters
///     ----------
///     raw_bytes :
///         Bytes to encode with RLP.
///
///     Returns
///     -------
///     encoded : `ethereum.base_types.Bytes`
///         The RLP encoded bytes representing `raw_bytes`.
///
pub fn encode_bytes(raw_bytes: &[u8]) -> Bytes {
    let len_raw_data = raw_bytes.len();
    if len_raw_data == 1 && raw_bytes[0] < 128 {
        return raw_bytes.into();
    } else if len_raw_data < 56 {
        [128 + len_raw_data as u8]
            .into_iter()
            .chain(raw_bytes.iter().copied())
            .collect()
    } else {
        let be_bytes = len_raw_data.to_be_bytes();
        let len_raw_data_as_be = strip_leading_zeros(&be_bytes);
        [183 + len_raw_data_as_be.len() as u8]
            .into_iter()
            .chain(len_raw_data_as_be.iter().copied())
            .chain(raw_bytes.iter().copied())
            .collect()
    }
}

pub fn encode_iter<T>(iter: impl IntoIterator<Item = T>) -> Bytes
    where
        T: EncodeRlp,
{
    let bytes = iter
        .into_iter()
        .flat_map(|item| encode(&item).to_vec())
        .collect::<Vec<_>>();
    encode_sequence(&bytes)
}

///
///     Encodes a list of RLP encodable objects (`raw_sequence`) using RLP.
///
///     Parameters
///     ----------
///     raw_sequence :
///             Sequence of RLP encodable objects.
///
///     Returns
///     -------
///     encoded : `ethereum.base_types.Bytes`
///         The RLP encoded bytes representing `raw_sequence`.
///
pub fn encode_sequence(joined_encodings: &[u8]) -> Bytes {
    // joined_encodings = get_joined_encodings(raw_sequence)?;
    let len_joined_encodings = joined_encodings.len();
    if len_joined_encodings < 56 {
        [192 + len_joined_encodings as u8]
            .into_iter()
            .chain(joined_encodings.iter().copied())
            .collect()
    } else {
        let be_bytes = len_joined_encodings.to_be_bytes();
        let len_joined_encodings_as_be = strip_leading_zeros(&be_bytes);
        [247 + len_joined_encodings_as_be.len() as u8]
            .into_iter()
            .chain(len_joined_encodings_as_be.iter().copied())
            .chain(joined_encodings.iter().copied())
            .collect()
    }
}
