use crate::{WyHash, _wyhash, wymix, P0, P1};
use core::hash::{BuildHasher, Hasher};

impl Hasher for WyHash {
    #[inline]
    fn finish(&self) -> u64 {
        wymix(P1 ^ self.size, self.h)
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if likely!(!bytes.is_empty()) {
            self.h = _wyhash(bytes, self.h);
            self.size += bytes.len() as u64
        } else {
            self.h ^= P0;
        }
    }
}

impl BuildHasher for WyHash {
    type Hasher = WyHash;

    fn build_hasher(&self) -> Self::Hasher {
        WyHash::with_seed(0)
    }
}

#[cfg(test)]
mod impl_tests {
    use crate::WyHash;

    #[test]
    fn adding_to_map() {
        use std::collections::HashMap;

        let hasher = WyHash::with_seed(0);
        let mut map: HashMap<String, String, WyHash> = HashMap::with_hasher(hasher);

        map.insert("Testing".to_string(), "value".to_string());

        assert!(map.contains_key("Testing"));
        assert!(map.get("Testing").unwrap() == "value")
    }
}
