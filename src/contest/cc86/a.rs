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
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut key = HashSet::<i32>::new();
        for i in 1..nums.len() {
            if key.get(&(nums[i] + nums[i - 1])).is_some() {
                return true;
            }
            key.insert(nums[i] + nums[i - 1]);
        }
        false
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
