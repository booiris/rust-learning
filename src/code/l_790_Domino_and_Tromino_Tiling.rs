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
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        const MOD: i64 = 1e9 as i64 + 7;
        let mut dp = vec![[0 as i64; 4]; n + 1];
        dp[1][0] = 1;
        dp[1][3] = 1;
        for i in 2..=n {
            dp[i][0] = (dp[i][0] + dp[i - 1][3]) % MOD;
            dp[i][1] = (dp[i][1] + dp[i - 1][0] + dp[i - 1][2]) % MOD;
            dp[i][2] = (dp[i][2] + dp[i - 1][0] + dp[i - 1][1]) % MOD;
            dp[i][3] = (dp[i][3] + dp[i - 1][0] + dp[i - 1][3] + dp[i - 1][2] + dp[i - 1][1]) % MOD;
            // println!("{:?}", dp[i]);
        }
        dp[n][3] as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", Solution::num_tilings(2));
}
