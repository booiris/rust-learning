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
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let (mut a, mut b) = (HashMap::new(), HashMap::new());
        for x in s.chars() {
            *a.entry(x).or_insert(0) += 1;
        }
        for x in target.chars() {
            *b.entry(x).or_insert(0) += 1;
        }
        let mut res = i32::MAX;
        for x in b {
            res = min(res, *a.entry(x.0).or_default() / x.1)
        }
        if res == i32::MAX {
            0
        } else {
            res
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
