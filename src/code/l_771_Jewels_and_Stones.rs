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
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let key = jewels.chars().collect::<HashSet<_>>();
        stones.chars().filter(|x| key.get(&x).is_some()).count() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
