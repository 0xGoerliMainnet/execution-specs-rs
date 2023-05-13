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

use super::base_types::{Bytes, Uint, strip_leading_zeros, U64, U32};

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
pub fn encode<R : EncodeRlp>(raw_data: &R) -> Bytes {
    raw_data.encode()
}

// if isinstance(raw_data, (bytearray, bytes))? {
//     return Ok(encode_bytes(raw_data)?);

impl EncodeRlp for Bytes {
    fn encode(&self) -> Bytes {
        encode_bytes(&self)
    }
}

impl<const N : usize> EncodeRlp for [u8; N] {
    fn encode(&self) -> Bytes {
        encode_bytes(self)
    }
}

// } else if isinstance(raw_data, (Uint, FixedUInt))? {
//     return Ok(encode(raw_data.to_be_bytes()?)?);

impl EncodeRlp for Uint {
    fn encode(&self) -> Bytes {
        let bytes = self.to_bytes_be();
        encode_bytes(&bytes)
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

impl<T : EncodeRlp> EncodeRlp for [T] {
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

impl<A : EncodeRlp> EncodeRlp for (A,) {
    fn encode(&self) -> Bytes {
        let mut joined_encodings = vec![];
        joined_encodings.extend(self.0.encode().iter().copied());
        encode_sequence(&joined_encodings)
    }
}

impl<A : EncodeRlp, B : EncodeRlp> EncodeRlp for (A, B) {
    fn encode(&self) -> Bytes {
        let mut joined_encodings = vec![];
        joined_encodings.extend(self.0.encode().iter().copied());
        joined_encodings.extend(self.1.encode().iter().copied());
        encode_sequence(&joined_encodings)
    }
}

impl<A : EncodeRlp, B : EncodeRlp, C : EncodeRlp> EncodeRlp for (A, B, C) {
    fn encode(&self) -> Bytes {
        let mut joined_encodings = vec![];
        joined_encodings.extend(self.0.encode().iter().copied());
        joined_encodings.extend(self.1.encode().iter().copied());
        joined_encodings.extend(self.2.encode().iter().copied());
        encode_sequence(&joined_encodings)
    }
}

impl<A : EncodeRlp, B : EncodeRlp, C : EncodeRlp, D : EncodeRlp> EncodeRlp for (A, B, C, D) {
    fn encode(&self) -> Bytes {
        let mut joined_encodings = vec![];
        joined_encodings.extend(self.0.encode().iter().copied());
        joined_encodings.extend(self.1.encode().iter().copied());
        joined_encodings.extend(self.2.encode().iter().copied());
        joined_encodings.extend(self.3.encode().iter().copied());
        encode_sequence(&joined_encodings)
    }
}

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
        [128 + len_raw_data as u8].into_iter().chain(raw_bytes.iter().copied()).collect()
    } else {
        let be_bytes = len_raw_data.to_be_bytes();
        let len_raw_data_as_be = strip_leading_zeros(&be_bytes);
        [183 + len_raw_data_as_be.len() as u8]
            .into_iter()
            .chain(len_raw_data_as_be.iter().copied())
            .chain(raw_bytes.iter().copied()).collect()
    }
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
        [192 + len_joined_encodings as u8].into_iter().chain(joined_encodings.iter().copied()).collect()
    } else {
        let be_bytes = len_joined_encodings.to_be_bytes();
        let len_joined_encodings_as_be = strip_leading_zeros(&be_bytes);
        [247 + len_joined_encodings_as_be.len() as u8]
            .into_iter()
            .chain(len_joined_encodings_as_be.iter().copied())
            .chain(joined_encodings.iter().copied()).collect()
    }
}

/*

/// 
///     Obtain concatenation of rlp encoding for each item in the sequence
///     raw_sequence.
/// 
///     Parameters
///     ----------
///     raw_sequence :
///         Sequence to encode with RLP.
/// 
///     Returns
///     -------
///     joined_encodings : `ethereum.base_types.Bytes`
///         The concatenated RLP encoded bytes for each item in sequence
///         raw_sequence.
///     
pub fn get_joined_encodings(raw_sequence: Sequence[RLP]) -> Result<Bytes, Error> {
    return Ok([].join(raw_sequence.iter().map(|item| encode(item)?).collect::<Vec<_>>())?);
}


/// 
///     Decodes an integer, byte sequence, or list of RLP encodable objects
///     from the byte sequence `encoded_data`, using RLP.
/// 
///     Parameters
///     ----------
///     encoded_data :
///         A sequence of bytes, in RLP form.
/// 
///     Returns
///     -------
///     decoded_data : `RLP`
///         Object decoded from `encoded_data`.
///     
pub fn decode(encoded_data: Bytes) -> Result<RLP, Error> {
    ensure(len(encoded_data)? > 0, RLPDecodingError("Cannot decode empty bytestring")?)?;
    if encoded_data[0] <= 191 {
        return Ok(decode_to_bytes(encoded_data)?);
    } else {
        return Ok(decode_to_sequence(encoded_data)?);
    }
}


T = TypeVar("T")?;
/// 
///     Decode the bytes in `encoded_data` to an object of type `cls`. `cls` can be
///     a `Bytes` subclass, a dataclass, `Uint`, `U256` or `Tuple[cls]`.
/// 
///     Parameters
///     ----------
///     cls: `Type[T]`
///         The type to decode to.
///     encoded_data :
///         A sequence of bytes, in RLP form.
/// 
///     Returns
///     -------
///     decoded_data : `T`
///         Object decoded from `encoded_data`.
///     
pub fn decode_to(cls: Type[T], encoded_data: Bytes) -> Result<T, Error> {
    return Ok(_decode_to(cls, decode(encoded_data)?)?);
}


/// 
///     Decode the rlp structure in `encoded_data` to an object of type `cls`.
///     `cls` can be a `Bytes` subclass, a dataclass, `Uint`, `U256`,
///     `Tuple[cls, ...]`, `Tuple[cls1, cls2]` or `Union[Bytes, cls]`.
/// 
///     Parameters
///     ----------
///     cls: `Type[T]`
///         The type to decode to.
///     raw_rlp :
///         A decoded rlp structure.
/// 
///     Returns
///     -------
///     decoded_data : `T`
///         Object decoded from `encoded_data`.
///     
pub fn _decode_to(cls: Type[T], raw_rlp: RLP) -> Result<T, Error> {
    if isinstance(cls, type(Tuple[Uint][...])?)? && cls._name == "Tuple" {
        ensure(type(raw_rlp)? == list, RLPDecodingError)?;
        if cls.__args__[1] == ... {
            args = [];
            for raw_item in raw_rlp {
                args.append(_decode_to(cls.__args__[0], raw_item)?)?;
            }
            return Ok(tuple(args)?);
        } else {
            args = [];
            ensure(len(raw_rlp)? == len(cls.__args__)?, RLPDecodingError)?;
            for (t, raw_item) in zip(cls.__args__, raw_rlp)? {
                args.append(_decode_to(t, raw_item)?)?;
            }
            return Ok(tuple(args)?);
        }
    } else if cls == Union[Bytes0][Bytes20] {
        ensure(type(raw_rlp)? == Bytes, RLPDecodingError)?;
        if len(raw_rlp)? == 0 {
            return Ok(Bytes0()?);
        } else if len(raw_rlp)? == 20 {
            return Ok(Bytes20(raw_rlp)?);
        } else {
            return Err(Error::RLPDecodingError("Bytes has length {}, expected 0 or 20".format(len(raw_rlp)?)?)?);
        }
    } else if isinstance(cls, type(List[Bytes])?)? && cls._name == "List" {
        ensure(type(raw_rlp)? == list, RLPDecodingError)?;
        items = [];
        for raw_item in raw_rlp {
            items.append(_decode_to(cls.__args__[0], raw_item)?)?;
        }
        return Ok(items);
    } else if isinstance(cls, type(Union[Bytes][List[Bytes]])?)? && cls.__origin__ == Union {
        if len(cls.__args__)? != 2 || !(Bytes).contains(cls.__args__) {
            return Err(Error::RLPDecodingError("RLP Decoding to type {} is not supported".format(cls)?)?);
        }
        if isinstance(raw_rlp, Bytes)? {
            return Ok(raw_rlp);
        } else if cls.__args__[0] == Bytes {
            return Ok(_decode_to(cls.__args__[1], raw_rlp)?);
        } else {
            return Ok(_decode_to(cls.__args__[0], raw_rlp)?);
        }
    } else if issubclass(cls, bool)? {
        if raw_rlp == [1] {
            return Ok(cls(true)?);
        } else if raw_rlp == [] {
            return Ok(cls(false)?);
        } else {
            return Err(Error::TypeError("Cannot decode {} as {}".format(raw_rlp, cls)?)?);
        }
    } else if issubclass(cls, FixedBytes)? {
        ensure(type(raw_rlp)? == Bytes, RLPDecodingError)?;
        ensure(len(raw_rlp)? == cls.LENGTH, RLPDecodingError)?;
        return Ok(raw_rlp);
    } else if issubclass(cls, Bytes)? {
        ensure(type(raw_rlp)? == Bytes, RLPDecodingError)?;
        return Ok(raw_rlp);
    } else if issubclass(cls, (Uint, FixedUInt))? {
        ensure(type(raw_rlp)? == Bytes, RLPDecodingError)?;
        // Try 
            return Ok(cls.from_be_bytes(raw_rlp)?);
    } else if is_dataclass(cls)? {
        ensure(type(raw_rlp)? == list, RLPDecodingError)?;
        assert!(isinstance(raw_rlp, list)?);
        args = [];
        ensure(len(fields(cls)?)? == len(raw_rlp)?, RLPDecodingError)?;
        for (field, rlp_item) in zip(fields(cls)?, raw_rlp)? {
            args.append(_decode_to(field.type, rlp_item)?)?;
        }
        return Ok(cls(*args)?);
    } else {
        return Err(Error::RLPDecodingError("RLP Decoding to type {} is not supported".format(cls)?)?);
    }
}


/// 
///     Decodes a rlp encoded byte stream assuming that the decoded data
///     should be of type `bytes`.
/// 
///     Parameters
///     ----------
///     encoded_bytes :
///         RLP encoded byte stream.
/// 
///     Returns
///     -------
///     decoded : `ethereum.base_types.Bytes`
///         RLP decoded Bytes data
///     
pub fn decode_to_bytes(encoded_bytes: Bytes) -> Result<Bytes, Error> {
    if len(encoded_bytes)? == 1 && encoded_bytes[0] < 128 {
        return Ok(encoded_bytes);
    } else if encoded_bytes[0] <= 183 {
        len_raw_data = encoded_bytes[0] - 128;
        ensure(len_raw_data < len(encoded_bytes)?, RLPDecodingError)?;
        raw_data = encoded_bytes[1..1 + len_raw_data];
        ensure(!(len_raw_data == 1 && raw_data[0] < 128), RLPDecodingError)?;
        return Ok(raw_data);
    } else {
        decoded_data_start_idx = 1 + encoded_bytes[0] - 183;
        ensure(decoded_data_start_idx - 1 < len(encoded_bytes)?, RLPDecodingError)?;
        ensure(encoded_bytes[1] != 0, RLPDecodingError)?;
        len_decoded_data = Uint.from_be_bytes(encoded_bytes[1..decoded_data_start_idx])?;
        ensure(len_decoded_data >= 56, RLPDecodingError)?;
        decoded_data_end_idx = decoded_data_start_idx + len_decoded_data;
        ensure(decoded_data_end_idx - 1 < len(encoded_bytes)?, RLPDecodingError)?;
        return Ok(encoded_bytes[decoded_data_start_idx..decoded_data_end_idx]);
    }
}


/// 
///     Decodes a rlp encoded byte stream assuming that the decoded data
///     should be of type `Sequence` of objects.
/// 
///     Parameters
///     ----------
///     encoded_sequence :
///         An RLP encoded Sequence.
/// 
///     Returns
///     -------
///     decoded : `Sequence[RLP]`
///         Sequence of objects decoded from `encoded_sequence`.
///     
pub fn decode_to_sequence(encoded_sequence: Bytes) -> Result<List[RLP], Error> {
    if encoded_sequence[0] <= 247 {
        len_joined_encodings = encoded_sequence[0] - 192;
        ensure(len_joined_encodings < len(encoded_sequence)?, RLPDecodingError)?;
        joined_encodings = encoded_sequence[1..1 + len_joined_encodings];
    } else {
        joined_encodings_start_idx = 1 + encoded_sequence[0] - 247;
        ensure(joined_encodings_start_idx - 1 < len(encoded_sequence)?, RLPDecodingError)?;
        ensure(encoded_sequence[1] != 0, RLPDecodingError)?;
        len_joined_encodings = Uint.from_be_bytes(encoded_sequence[1..joined_encodings_start_idx])?;
        ensure(len_joined_encodings >= 56, RLPDecodingError)?;
        joined_encodings_end_idx = joined_encodings_start_idx + len_joined_encodings;
        ensure(joined_encodings_end_idx - 1 < len(encoded_sequence)?, RLPDecodingError)?;
        joined_encodings = encoded_sequence[joined_encodings_start_idx..joined_encodings_end_idx];
    }
    return Ok(decode_joined_encodings(joined_encodings)?);
}


/// 
///     Decodes `joined_encodings`, which is a concatenation of RLP encoded
///     objects.
/// 
///     Parameters
///     ----------
///     joined_encodings :
///         concatenation of RLP encoded objects
/// 
///     Returns
///     -------
///     decoded : `List[RLP]`
///         A list of objects decoded from `joined_encodings`.
///     
pub fn decode_joined_encodings(joined_encodings: Bytes) -> Result<List[RLP], Error> {
    decoded_sequence = [];
    item_start_idx = 0;
    while item_start_idx < len(joined_encodings)? {
        encoded_item_length = decode_item_length(joined_encodings[item_start_idx..])?;
        ensure(item_start_idx + encoded_item_length - 1 < len(joined_encodings)?, RLPDecodingError)?;
        encoded_item = joined_encodings[item_start_idx..item_start_idx + encoded_item_length];
        decoded_sequence.append(decode(encoded_item)?)?;
        item_start_idx += encoded_item_length;
    }
    return Ok(decoded_sequence);
}


/// 
///     Find the length of the rlp encoding for the first object in the
///     encoded sequence.
///     Here `encoded_data` refers to concatenation of rlp encoding for each
///     item in a sequence.
/// 
///     NOTE - This is a helper function not described in the spec. It was
///     introduced as the spec doesn't discuss about decoding the RLP encoded
///     data.
/// 
///     Parameters
///     ----------
///     encoded_data :
///         RLP encoded data for a sequence of objects.
/// 
///     Returns
///     -------
///     rlp_length : `int`
///     
pub fn decode_item_length(encoded_data: Bytes) -> Result<int, Error> {
    ensure(len(encoded_data)? > 0, RLPDecodingError)?;
    first_rlp_byte = Uint(encoded_data[0])?;
    length_length = Uint(0)?;
    decoded_data_length = 0;
    if first_rlp_byte < 128 {
        return Ok(1);
    } else if first_rlp_byte <= 183 {
        decoded_data_length = first_rlp_byte - 128;
    } else if first_rlp_byte <= 191 {
        length_length = first_rlp_byte - 183;
        ensure(length_length < len(encoded_data)?, RLPDecodingError)?;
        ensure(encoded_data[1] != 0, RLPDecodingError)?;
        decoded_data_length = Uint.from_be_bytes(encoded_data[1..1 + length_length])?;
    } else if first_rlp_byte <= 247 {
        decoded_data_length = first_rlp_byte - 192;
    } else if first_rlp_byte <= 255 {
        length_length = first_rlp_byte - 247;
        ensure(length_length < len(encoded_data)?, RLPDecodingError)?;
        ensure(encoded_data[1] != 0, RLPDecodingError)?;
        decoded_data_length = Uint.from_be_bytes(encoded_data[1..1 + length_length])?;
    }
    return Ok(1 + length_length + decoded_data_length);
}


/// 
///     Obtain the keccak-256 hash of the rlp encoding of the passed in data.
/// 
///     Parameters
///     ----------
///     data :
///         The data for which we need the rlp hash.
/// 
///     Returns
///     -------
///     hash : `Hash32`
///         The rlp hash of the passed in data.
///     
pub fn rlp_hash(data: RLP) -> Result<Hash32, Error> {
    return Ok(keccak256(encode(data)?)?);
}


*/
