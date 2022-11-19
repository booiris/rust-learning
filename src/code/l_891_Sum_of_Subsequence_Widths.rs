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
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        const MOD: i64 = 1_000_000_007;
        let mut high: i64 = 0;
        let mut low: i64 = 0;
        let n = nums.len();
        for i in 0..nums.len() {
            low = (low * 2 + nums[i] as i64) % MOD;
            high = (high * 2 + nums[n - i - 1] as i64) % MOD;
        }
        ((high + MOD - low) % MOD) as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![2, 1, 3];
    println!("res:{}", Solution::sum_subseq_widths(a));
}
