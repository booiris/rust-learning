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
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
