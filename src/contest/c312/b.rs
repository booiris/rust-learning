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
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let nums = &nums;
        let key = nums.into_iter().max().unwrap();
        let mut cnt = 0;
        let mut res = 0;
        for i in nums {
            if i == key {
                cnt += 1;
            } else {
                cnt = 0;
            }
            res = max(res, cnt);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
