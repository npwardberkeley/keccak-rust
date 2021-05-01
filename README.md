# keccak-rust

An implementation of keccak functions.
[`The Keccak reference`](https://keccak.team/files/Keccak-reference-3.0.pdf).

# Example

```toml
[dependencies]
keccak-rust = *
```

```rust
extern crate keccak_rust;
use keccak_rust::*;

const YOUR_INPUT_BYTES: [Byte; 12] = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];

fn main() {
    let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
    keccak.append(&mut YOUR_INPUT_BYTES);
    println!("{:?}", keccak.hash());
}
```
