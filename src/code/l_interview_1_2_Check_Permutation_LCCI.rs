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
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut key = HashMap::new();
        for x in s1.chars() {
            key.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }
        for x in s2.chars() {
            let temp = key.get_mut(&x);
            if let Some(y) = temp {
                *y -= 1;
                if *y < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
