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
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut key = heights
            .iter()
            .zip(names.iter())
            .map(|(&x, y)| (Reverse(x), y.clone()))
            .collect::<Vec<_>>();
        key.sort_unstable();
        key.iter().map(|x| x.1.clone()).collect::<Vec<_>>()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
