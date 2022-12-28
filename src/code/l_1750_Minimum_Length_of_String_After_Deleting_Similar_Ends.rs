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
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j && s[i] == s[j] {
            let now = s[i];
            while i <= j && s[i] == now {
                i += 1;
            }
            while i <= j && s[j] == now {
                j -= 1;
            }
        }
        j as i32 - i as i32 + 1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let s = "cabaabac";
    println!("res:{}", Solution::minimum_length(s.into()));
}
