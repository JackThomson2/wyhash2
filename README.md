# WyHash2

A simple, light and fast hashing algorithm written by [Wang Yi](https://github.com/wangyi-fudan/wyhash). Ported to rust for your ease of use.

Thanks also goes to [Eldruin](https://github.com/eldruin/wyhash-rs) their WyHash was very helpful for reference on getting started.

## Features

- `nightly` which enables some nightly only intrinsicts which can help improve performance.

 - `std` This is enabled by default, however can be disabled for `no_std` enviroments

## Usage

A basic example:

```rust
use core::hash::Hasher;
use wyhash2::WyHash;

fn main() {
    let secret = 0;
    let mut hasher = WyHash::with_seed(secret);
    hasher.write(&[0, 1, 2]);
    println!("We got {}", hasher.finish());
}
```
**Wyhash2** Implements *BuildHasher* which means it can be used as the hasher for the Hashmap

Usage with a Hashmap:

```rust
use wyhash2::WyHash;
use std::collections::HashMap;

fn main() {
    let hasher = WyHash::with_seed(0);
    let mut map: HashMap<String, String, WyHash> = HashMap::with_hasher(hasher);

    map.insert("Hello".to_string(), "World".to_string());

    println!("We got {}", map.get("Hello").unwrap());
}
```

## no_std usage

``` toml
[dependencies]
wyhash2 = { version = "...", default-features = false }
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.