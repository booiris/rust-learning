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
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut s1 = s1.chars().collect::<Vec<_>>();
        let mut s2 = s2.chars().collect::<Vec<_>>();
        for i in 0..s1.len() {
            for j in 0..s1.len() {
                s1.swap(i, j);
                if s1 == s2 {
                    return true;
                }
                s1.swap(i, j);
            }
        }
        false
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
