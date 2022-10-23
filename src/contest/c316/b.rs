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

fn gcd(m: i32, n: i32) -> i32 {
    // assert!(m != 0 && n != 0);
    if n == 0 {
        return m;
    }

    return gcd(n, m % n);
}

impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            let mut now = nums[i];
            for j in i..nums.len() {
                now = gcd(now, nums[j]);
                if now == k {
                    res += 1;
                }
                if now == 0 {
                    break;
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
