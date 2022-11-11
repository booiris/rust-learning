#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
#[cfg(feature = "local")]
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.to_ascii_lowercase();
        let s = s.into_bytes();
        let mut key1 = 0;
        for i in 0..s.len() / 2 {
            for x in ['a', 'e', 'i', 'o', 'u'] {
                let x = x as u8;
                if x == s[i] {
                    key1 += 1;
                }
            }
        }
        let mut key2 = 0;
        for i in 0..s.len() / 2 {
            for x in ['a', 'e', 'i', 'o', 'u'] {
                let x = x as u8;
                if x == s[i + s.len() / 2] {
                    key2 += 1;
                }
            }
        }
        key1 == key2
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
