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
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; arr.len() + 1];
        for (i, x) in arr.iter().enumerate() {
            let mut maxn = x;
            for j in 0..k {
                if i < j {
                    break;
                }
                maxn = maxn.max(&arr[i - j]);
                dp[i + 1] = dp[i + 1].max(dp[i - j] + (j + 1) as i32 * *maxn);
            }
        }
        dp[arr.len()]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let arr = [1, 15, 7, 9, 2, 5, 10];
    println!(
        "res:{}",
        Solution::max_sum_after_partitioning(arr.into(), 3)
    );
}
