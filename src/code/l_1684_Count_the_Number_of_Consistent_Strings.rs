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
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut key = HashSet::new();
        for x in allowed.chars() {
            key.insert(x);
        }
        let mut res = 0;
        for x in words {
            let mut flag = true;
            for y in x.chars() {
                if key.get(&y).is_none() {
                    flag = false;
                    break;
                }
            }
            if flag {
                res += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
