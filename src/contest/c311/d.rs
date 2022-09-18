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

// impl Solution {
//     pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {}
// }

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", i32::max(1, 2));
}
