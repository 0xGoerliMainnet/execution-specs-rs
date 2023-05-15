use num_traits::ToPrimitive;

use super::{
    base_types::Uint,
    frontier::fork_types::{keccak256, keccak512, Hash32}, utils::numeric::{le_uint32_sequence_to_uint, le_bytes_to_uint32_sequence, le_uint32_sequence_to_bytes},
};

const EPOCH_SIZE: u64 = 30_000;
const INITIAL_CACHE_SIZE: u64 = 1 << 24;
const CACHE_EPOCH_GROWTH_SIZE: u64 = 1 << 17;
const INITIAL_DATASET_SIZE: u64 = 1 << 30;
const DATASET_EPOCH_GROWTH_SIZE: u64 = 1 << 23;
const HASH_BYTES: usize = 64;
const MIX_BYTES: usize = 128;
const CACHE_ROUNDS: usize = 3;
const DATASET_PARENTS: usize = 256;
const HASHIMOTO_ACCESSES: usize = 64;

fn epoch(block_number: u64) -> u64 {
    block_number / EPOCH_SIZE
}

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u64) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn cache_size(block_number: u64) -> u64 {
    let mut size = INITIAL_CACHE_SIZE + (CACHE_EPOCH_GROWTH_SIZE * epoch(block_number));
    size -= HASH_BYTES as u64;
    while !is_prime(size / (HASH_BYTES as u64)) {
        size -= 2 * (HASH_BYTES as u64);
    }
    size
}

fn dataset_size(block_number: u64) -> u64 {
    let mut size = INITIAL_DATASET_SIZE + (DATASET_EPOCH_GROWTH_SIZE * epoch(block_number));
    size -= MIX_BYTES as u64;
    while !is_prime(size / (MIX_BYTES as u64)) {
        size -= 2 * (MIX_BYTES as u64);
    }
    size
}

fn generate_seed(block_number: u64) -> Hash32 {
    let mut epoch_number = epoch(block_number);

    let mut seed = Hash32::default();
    while epoch_number != 0 {
        seed = keccak256(&seed);
        epoch_number -= 1;
    }

    seed
}

fn generate_cache(block_number: u64) -> Vec<Vec<u32>> {
    let seed = generate_seed(block_number);
    let cache_size_bytes = cache_size(block_number);

    let cache_size_words = cache_size_bytes / HASH_BYTES as u64;
    let mut cache = vec![keccak512(&seed)];

    let mut previous_cache_item = cache[0].clone();
    for _ in 1..cache_size_words {
        let cache_item = keccak512(&previous_cache_item);
        cache.push(cache_item.clone());
        previous_cache_item = cache_item;
    }

    for _ in 0..CACHE_ROUNDS {
        for index in 0..cache_size_words {
            let first_cache_item = cache
                [((index.wrapping_sub(1) + cache_size_words) % cache_size_words) as usize]
                .clone();
            let second_cache_item = cache[u32::from_le_bytes([
                cache[index as usize][0],
                cache[index as usize][1],
                cache[index as usize][2],
                cache[index as usize][3],
            ]) as usize
                % cache_size_words as usize]
                .clone();
            let result: Vec<u8> = first_cache_item
                .iter()
                .zip(second_cache_item.iter())
                .map(|(a, b)| a ^ b)
                .collect();
            cache[index as usize] = keccak512(&result);
        }
    }

    cache
        .into_iter()
        .map(|cache_item| {
            cache_item
                .chunks_exact(4)
                .map(|chunk| u32::from_le_bytes(TryFrom::try_from(chunk).unwrap()))
                .collect()
        })
        .collect()
}

fn fnv(a: u32, b: u32) -> u32 {
    const FNV_PRIME: u32 = 0x0100_0193;
    const U32_MAX_VALUE: u32 = std::u32::MAX;

    let a = u64::from(a);
    let b = u64::from(b);

    let result = ((a * FNV_PRIME as u64) ^ b) & U32_MAX_VALUE as u64;
    result.to_u32().unwrap()
}

fn fnv_hash(mix_integers: &[u32], data: &[u32]) -> Vec<u32> {
    mix_integers
        .iter()
        .enumerate()
        .map(|(i, &mix_integer)| fnv(mix_integer, data[i]))
        .collect()
}

fn generate_dataset_item(cache: &Vec<Vec<u32>>, index: usize) -> Vec<u8> {
    let mix = keccak512(
        (le_uint32_sequence_to_uint(&cache[index % cache.len()]).unwrap() ^ Uint::from(index))
            .to_bytes_le().as_slice(),
    );

    let mut mix_integers = le_bytes_to_uint32_sequence(&mix);

    for j in 0..DATASET_PARENTS {
        let mix_word = mix_integers[j % 16];
        let cache_index = fnv((index ^ j) as u32, mix_word) % cache.len() as u32;
        let parent = &cache[cache_index as usize];
        mix_integers = fnv_hash(&mix_integers, parent);
    }

    let mix = le_uint32_sequence_to_bytes(&mix_integers).unwrap();

    keccak512(Box::leak(mix)).to_vec()
}

fn generate_dataset(block_number: u64) -> Vec<Vec<u8>> {
    let dataset_size_bytes = dataset_size(block_number);
    let cache = generate_cache(block_number);

    (0..(dataset_size_bytes / HASH_BYTES as u64))
        .map(|index| generate_dataset_item(&cache, index as usize))
        .collect()
}