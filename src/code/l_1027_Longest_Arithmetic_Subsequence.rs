#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
use std::vec;
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
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let maxn = *nums.iter().max().unwrap() as usize;
        let minn = *nums.iter().min().unwrap();
        let diff = maxn as i32 - minn;
        let mut dp = vec![-1; maxn + 1];
        for d in -diff..=diff {
            dp.clear();
            dp.resize(maxn + 1, -1);
            for &x in &nums {
                let prev = x - d;
                let x = x as usize;
                if prev >= minn && prev <= maxn as i32 && dp[prev as usize] != -1 {
                    dp[x] = max(dp[x], dp[prev as usize] + 1);
                    res = max(res, dp[x]);
                }
                dp[x] = max(dp[x], 1);
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
