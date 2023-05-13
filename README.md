# execution-specs-rs

`ETHGlobal Lisbon Hackathon` 

The puspose of execution-specs is to further understand how transactions work on the EVM for anyone who wants to create an EIP or develop their own layer 1. Most client nodes, whether layer 1 or 2, are build in Go Lang or Rust for performance reasons, but they are very implementation specific and suffer from legacy codebases. If the execution specs were available in these languages it would allow client node developers to start from a better place as opposed to copying and pasting existing codebases. 

There are over 90k tests written in Python that take over 30mins to run. We have re-implemented this in Rust with documentation for the purpose of learning and educating developers on how Ethereum works. The docs are generated with cargo docs and the output is the docs.rs format, which is accessible to Rust developers.


## get started

Rust translation of frontier ethereum execution specs.

To generate the code into rust-execution-specs

```
cargo run --bin pytran
```

After that it is up to you!

This will create the rust-execution-specs directory.

We will remove this from .gitignore in Lisbon.


## Battle plan for eth-Lisbon

To translate as much as is practical into idiomatic working Rust
code.

Use BigInt for now as the integer type (int) - we can make this better later.
Where appropriate, use u64 or usize.

Use Box<[u8]> for Bytes.

The main.rs in pytran is set up to only translate the roots of the
src/ and tests/ dirs. We can add more as required.


Suggested starting points:

* exceptions.rs     This defines the error. ie. enum Error {...}
* base_types.rs     Make equivalent types to Bytes8 etc. (see eth_arrow for examples.)
* genesis.rs        Be able to read the mainnet.json file.
* ethash.rs         Skip this.
* rlp.rs            Convert _decode_to to a trait and implement it for various types.
* tests/test_rlp.rs Make this work.

Note that until you add the mod to the mod.rs, nothing will attempt to compile.
Do these one at a time and only check in when the project compiles.
