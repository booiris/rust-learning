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
    pub fn are_numbers_ascending(s: String) -> bool {
        s.split_ascii_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|x| x[0] < x[1])
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
