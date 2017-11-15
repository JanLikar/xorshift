# Xorshift

This crate contains an implementation of the Xorshift32 pseudorandom number
generator. Xorshift PRNGs are relatively fast, because they rely mostly on
bitwise shift and xor operations.

Xorshift32 has a period of 2^32 − 1.

## Usage 

**Should not be used for cryptographic applications!**

```
extern crate Xorshift;

use Xorshift;

// The seed is a non-zero u32.
let shifter = Xorshift::new(seed);

println!('{:?}', shifter.take(10).collect());
```
