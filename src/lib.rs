#![feature(core_intrinsics)]

use core::hash::Hasher;
use std::intrinsics::{likely, unlikely};
use std::slice;

const P0: u64 = 0xa076_1d64_78bd_642f;
const P1: u64 = 0xe703_7ed1_a0b4_28db;
const P2: u64 = 0x8ebc_6af0_9c88_c6e3;
const P3: u64 = 0x5899_65cc_7537_4cc3;

#[inline]
fn as_array_8(slice: &[u8]) -> &[u8; 8] {
    debug_assert!(slice.len() == 8);
    unsafe { &*(slice.as_ptr() as *const [_; 8]) }
}

#[inline]
fn as_array_4(slice: &[u8]) -> &[u8; 4] {
    debug_assert!(slice.len() == 4);
    unsafe { &*(slice.as_ptr() as *const [_; 4]) }
}

#[inline]
fn wyr8(data: &[u8; 8]) -> u64 {
    u64::from_ne_bytes(*data)
}

#[inline]
fn wyr4(data: &[u8; 4]) -> u64 {
    u32::from_ne_bytes(*data) as u64
}

#[inline]
fn wyr3(data: &[u8], k: usize) -> u64 {
    debug_assert!(k >> 1 <= data.len());
    unsafe {
        ((*data.get_unchecked(0) as u64) << 16)
            | ((*data.get_unchecked(k >> 1) as u64) << 8)
            | (*data.get_unchecked(k - 1) as u64)
    }
}

#[inline]
fn wymum(a: &mut u64, b: &mut u64) {
    let r = u128::from(*a) * u128::from(*b);

    *a = r as u64;
    *b = (r >> 64) as u64;
}

#[inline]
fn wymix(mut a: u64, mut b: u64) -> u64 {
    wymum(&mut a, &mut b);

    a ^ b
}

#[inline]
pub fn _wyhash(bytes: &[u8], mut seed: u64) -> u64 {
    seed ^= P0;

    let a: u64;
    let b: u64;

    if likely(bytes.len() <= 16) {
        if likely(bytes.len() <= 8) {
            if likely(bytes.len() >= 4) {
                a = wyr4(as_array_4(&bytes[0..4]));
                b = wyr4(as_array_4(&bytes[bytes.len() - 4..]));
                return wymix(a ^ P1, b ^ seed);
            } else if likely(!bytes.is_empty()) {
                a = wyr3(&bytes[..], bytes.len());
                b = 0;
                return wymix(a ^ P1, b ^ seed);
            } else {
                a = 0;
                b = 0;
                return wymix(a ^ P1, b ^ seed);
            }
        } else {
            a = wyr8(as_array_8(&bytes[0..8]));
            b = wyr8(as_array_8(&bytes[bytes.len() - 9..]));
            return wymix(a ^ P1, b ^ seed);
        }
    } else {
        let mut i = bytes.len();

        let ptr = bytes.as_ptr();
        let mut pos = 0;
        if unlikely(bytes.len() > 48) {
            let mut see1 = seed;
            let mut see2 = seed;
            while i > 48 {
                unsafe {
                    seed = wymix(
                        wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos), 8))) ^ P1,
                        wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos + 8), 8))) ^ seed,
                    );

                    see1 = wymix(
                        wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos + 16), 8))) ^ P2,
                        wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos + 24), 8))) ^ see1,
                    );

                    see2 = wymix(
                        wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos + 32), 8))) ^ P3,
                        wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos + 40), 8))) ^ see2,
                    );
                }
                pos += 48;
                i -= 48;
            }
            seed ^= see1 ^ see2;
        }
        while unlikely(i > 16) {
            unsafe {
                seed = wymix(
                    wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos), 8))) ^ P1,
                    wyr8(as_array_8(slice::from_raw_parts(ptr.add(pos + 8), 8))) ^ seed,
                );
            }
            pos += 16;
            i -= 16;
        }

        unsafe {
            let offset = pos + i - 16;
            a = wyr8(as_array_8(slice::from_raw_parts(ptr.add(offset), 8)));

            let offset = pos + i - 8;
            b = wyr8(as_array_8(slice::from_raw_parts(ptr.add(offset), 8)));
        }
    }
    wymix(a ^ P1, b ^ seed)
}

pub fn wyhash_single(bytes: &[u8], seed: u64) -> u64 {
    let seed = _wyhash(bytes, seed);
    wymix(P1 ^ bytes.len() as u64, seed)
}

/// WyHash hasher
#[derive(Default, Clone, Copy)]
pub struct WyHash {
    h: u64,
    size: u64,
}

impl WyHash {
    /// Create hasher with a seed
    pub fn with_seed(seed: u64) -> Self {
        WyHash { h: seed, size: 0 }
    }
}

impl Hasher for WyHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if likely(!bytes.is_empty()) {
            self.h = _wyhash(bytes, self.h);
            self.size += bytes.len() as u64
        } else {
            self.h ^= P0;
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        wymix(P1 ^ self.size, self.h)
    }
}

#[cfg(test)]
mod tests {
    use crate::wyhash_single;

    #[test]
    fn it_works() {
        let data: [u8; 80] = [1; 80];

        println!("input {} data {}", 'a' as u8, wyhash_single(&data, 1));
        assert!(wyhash_single(&data, 1) != 0)
    }
}
