// /// 
// /// Ethereum Logs Bloom
// /// ^^^^^^^^^^^^^^^^^^^
// /// 
// /// .. contents:: Table of Contents
// ///     :backlinks: none
// ///     :local:
// /// 
// /// Introduction
// /// ------------
// /// 
// /// This modules defines functions for calculating bloom filters of logs. For the
// /// general theory of bloom filters see e.g. `Wikipedia
// /// <https://en.wikipedia.org/wiki/Bloom_filter>`_. Bloom filters are used to allow
// /// for efficient searching of logs by address and/or topic, by rapidly
// /// eliminating blocks and reciepts from their search.
// /// 
// use ::typing::{Tuple};
// use ::ethereum::base_types::{Uint};
// use ::ethereum::crypto::hash::{keccak256};
// use super::fork_types::{Bloom, Log};
// /// 
// ///     Add a bloom entry to the bloom filter (`bloom`).
// /// 
// ///     The number of hash functions used is 3. They are calculated by taking the
// ///     least significant 11 bits from the first 3 16-bit words of the
// ///     `keccak_256()` hash of `bloom_entry`.
// /// 
// ///     Parameters
// ///     ----------
// ///     bloom :
// ///         The bloom filter.
// ///     bloom_entry :
// ///         An entry which is to be added to bloom filter.
// ///     
// pub fn add_to_bloom(bloom: bytearray, bloom_entry: bytes) -> Result<(), Error> {
//     hash = keccak256(bloom_entry)?;
//     for idx in (0, 2, 4) {
//         bit_to_set = Uint.from_be_bytes(hash[idx..idx + 2])? & 2047;
//         bit_index = 2047 - bit_to_set;
//         byte_index = (bit_index).floordiv(8);
//         bit_value = 1 << 7 - bit_index % 8;
//         bloom[byte_index] = bloom[byte_index] | bit_value;
//     }
// }


// /// 
// ///     Obtain the logs bloom from a list of log entries.
// /// 
// ///     The address and each topic of a log are added to the bloom filter.
// /// 
// ///     Parameters
// ///     ----------
// ///     logs :
// ///         List of logs for which the logs bloom is to be obtained.
// /// 
// ///     Returns
// ///     -------
// ///     logs_bloom : `Bloom`
// ///         The logs bloom obtained which is 256 bytes with some bits set as per
// ///         the caller address and the log topics.
// ///     
// pub fn logs_bloom(logs: Tuple[Log][...]) -> Result<Bloom, Error> {
//     // TypedAssignment unsupported
//     for log in logs {
//         add_to_bloom(bloom, log.address)?;
//         for topic in log.topics {
//             add_to_bloom(bloom, topic)?;
//         }
//     }
//     return Ok(Bloom(bloom)?);
// }


