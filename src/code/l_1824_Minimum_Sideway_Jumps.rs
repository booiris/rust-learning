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
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        // TODO
        let mut dp = vec![vec![0; 3]; obstacles.len() + 5];
        dp[0][0] = i32::MAX / 2;
        dp[0][2] = i32::MAX / 2;
        for i in 1..=obstacles.len() {
            if obstacles[i - 1] != 1 {
                dp[i][0] = min(dp[i - 1][0], min(dp[i - 1][1], dp[i - 1][2]) + 1);
            } else {
                dp[i][0] = i32::MAX / 2;
            }
            if obstacles[i - 1] != 2 {
                dp[i][1] = min(dp[i - 1][1], min(dp[i - 1][0], dp[i - 1][2]) + 1);
            } else {
                dp[i][1] = i32::MAX / 2;
            }
            if obstacles[i - 1] != 3 {
                dp[i][2] = min(dp[i - 1][2], min(dp[i - 1][0], dp[i - 1][1]) + 1);
            } else {
                dp[i][2] = i32::MAX / 2;
            }
            println!("{:?}",dp[i]);
        }
        *dp[obstacles.len()].iter().min().unwrap()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = [0, 1, 2, 3, 0];
    println!("res:{}", Solution::min_side_jumps(a.into()));
}
