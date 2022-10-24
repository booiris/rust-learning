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
    pub fn partition_disjoint1(nums: Vec<i32>) -> i32 {
        let mut maxn = vec![-1; nums.len() + 1];
        let mut minn = vec![i32::MAX; nums.len() + 1];
        for i in 1..=nums.len() {
            maxn[i] = maxn[i - 1].max(nums[i - 1]);
        }
        for i in (1..=nums.len()).rev() {
            minn[i - 1] = minn[i].min(nums[i - 1]);
        }
        for i in 1..=nums.len() {
            if maxn[i] <= minn[i] {
                return i as i32;
            }
        }
        0
    }
}

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut leftmax = nums[0];
        let mut curmax = nums[0];
        for i in 1..nums.len() - 1 {
            curmax = curmax.max(nums[i]);
            if nums[i] < leftmax {
                res = i + 1;
                leftmax = curmax;
            }
        }
        res as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
