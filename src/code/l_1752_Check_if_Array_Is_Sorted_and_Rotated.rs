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
    pub fn check(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut res = 0;
        for i in 0..len {
            if nums[i] > nums[(i + 1) % len] {
                res += 1;
            }
        }
        return res < 2;
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![7, 9, 1, 1, 1, 3];
    println!("res:{}", Solution::check(a));
}
