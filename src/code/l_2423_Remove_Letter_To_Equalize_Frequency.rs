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
        let mut key = HashMap::new();
        for x in word.chars() {
            *key.entry(x).or_insert(0) += 1;
        }
        let mut res = -1;
        let mut end = false;
        for (_, v) in key {
            if res == -1 {
                res = v;
            } else if v != res {
                if res + 1 != v || end {
                    return false;
                } else {
                    end = true;
                }
            }
        }
        if res == 1 {
            return true;
        }
        end
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
