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
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        let mut res = 0;
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = -2;
    println!("res:{}", Solution::reach_number(a));
}
