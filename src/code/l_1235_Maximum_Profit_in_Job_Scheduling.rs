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
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut kk = end_time.clone();
        kk.sort_unstable();
        let mut key = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|(x, y)| (x, y))
            .collect::<Vec<_>>();
        key.sort_unstable_by_key(|x| x.0 .1);
        let mut dp = vec![0; key.len() + 1];
        for i in 1..=key.len() {
            let mut k = kk.partition_point(|&x| x <= key[i - 1].0 .0);
            if k == i {
                k -= 1;
            }
            dp[i] = dp[i - 1].max(dp[k] + key[i - 1].1);
        }
        dp[key.len()]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 2, 3, 3];
    let b = vec![3, 4, 5, 6];
    let c = vec![50, 10, 40, 70];
    println!("res:{}", Solution::job_scheduling(a, b, c));
}
