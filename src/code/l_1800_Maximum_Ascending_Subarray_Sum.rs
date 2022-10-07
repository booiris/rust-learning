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
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut pre = -1;
        let mut now = 0;
        for x in nums {
            if x > pre {
                now += x;
            } else {
                now = x;
            }
            res = res.max(now);
            pre = x;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
