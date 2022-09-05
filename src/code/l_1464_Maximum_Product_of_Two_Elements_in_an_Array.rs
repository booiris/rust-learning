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
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                res = max(res, (x - 1) * (y - 1));
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let i = vec![3, 4, 5, 2];
    println!("res:{}", Solution::max_product(i));
}
