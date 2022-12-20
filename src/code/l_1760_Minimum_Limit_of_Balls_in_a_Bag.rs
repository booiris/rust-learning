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

fn check(aim: i32, nums: &[i32], mut max_op: i32) -> bool {
    for &x in nums {
        if x > aim {
            let mut k = x / aim;
            if x % aim == 0 {
                k -= 1;
            }
            max_op -= k;
            if max_op < 0 {
                return false;
            }
        }
    }
    true
}

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut low = 1;
        let mut high = i32::MAX / 2;
        while low <= high {
            let mid = low + (high - low) / 2;
            if check(mid, &nums, max_operations) {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let n = vec![2, 4, 8, 2];
    println!("res:{}", Solution::minimum_size(n, 4));
}
