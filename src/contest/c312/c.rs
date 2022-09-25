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
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut k1 = vec![1; nums.len()];
        let mut k2 = vec![1; nums.len()];
        for (i, x) in nums.iter().enumerate() {
            if i != 0 && nums[i - 1] >= *x {
                k1[i] = k1[i - 1] + 1;
            }
        }
        for i in (0..nums.len()).rev() {
            if i != nums.len() - 1 && nums[i + 1] >= nums[i] {
                k2[i] = k2[i + 1] + 1;
            }
        }
        let mut res = vec![];
        for i in k..nums.len() as i32 - k {
            let i = i as usize;
            if k1[i - 1] >= k && k2[i + 1] >= k {
                res.push(i as i32);
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
