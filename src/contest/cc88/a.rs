#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::hash::Hash;
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
    pub fn equal_frequency(word: String) -> bool {
        for i in 0..word.len() {
            let mut cnt = HashMap::new();
            for (j, x) in word.chars().enumerate() {
                if i == j {
                    continue;
                }
                *cnt.entry(x).or_insert(0) += 1;
            }
            let mut key = HashSet::new();
            for x in cnt.iter() {
                key.insert(*x.1);
            }
            if key.len() <= 1 {
                return true;
            }
        }
        false
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
