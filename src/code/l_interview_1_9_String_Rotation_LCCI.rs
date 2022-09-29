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
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        s1.len() == s2.len() && s2.repeat(2).contains(&s1)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!(
        "res:{}",
        Solution::is_fliped_string("waterbottle".to_string(), "erbottlewat".to_string())
    );
}
