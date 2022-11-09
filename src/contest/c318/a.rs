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
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] <<= 1;
                nums[i + 1] = 0;
            }
        }
        let mut res = vec![0; nums.len()];
        let mut now = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                res[now] = nums[i];
                now += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
