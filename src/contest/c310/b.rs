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
    pub fn partition_string(s: String) -> i32 {
        let mut res = 0;
        let mut key = HashSet::new();
        for x in s.chars() {
            if key.get(&x).is_some() {
                res += 1;
                key.clear();
            }
            key.insert(x);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
