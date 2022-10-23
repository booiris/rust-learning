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
    pub fn make_similar(mut nums: Vec<i32>, mut target: Vec<i32>) -> i64 {
        let mut res = 0;
        nums.sort_unstable();
        target.sort_unstable();
        let mut i = 0;
        let mut j = target.len() - 1;
        let mut k1 = vec![];
        let mut k2 = vec![];
        for x in 0..nums.len() {
            if nums[x] > target[x] {
                k1.push(nums[x] as i64 - target[x] as i64);
            } else if nums[x] < target[x] {
                k2.push(x);
            }
        }
        let mut res = k1.iter().sum::<i64>() / 2;
        if k1.iter().sum::<i64>() % 2 != 0 {
            res += 1;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
