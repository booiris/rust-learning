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
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut key = HashMap::new();
        for x in nums {
            if x % 2 == 0 {
                key.entry(x).and_modify(|c| *c += 1).or_insert(1);
            }
        }
        let mut res = -1;
        let mut cnt = 0;
        for x in key {
            if cnt < x.1 {
                cnt = x.1;
                res = x.0;
            }
            if cnt == x.1 {
                if res == -1 {
                    res = x.0;
                }
                res = min(res, x.0);
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
