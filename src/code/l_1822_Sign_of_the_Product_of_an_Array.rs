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
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        nums.iter().for_each(|&x| match x {
            x if x < 0 => res = -res,
            x if x > 0 => res = res,
            _ => res = 0,
        });
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
