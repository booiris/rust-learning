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
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![0; 26]; n + 1];
        const MOD: i32 = 1e9 as i32 + 7;

        for (i, x) in s.chars().enumerate() {
            let i = i + 1;
            let x = (x as u8 - 'a' as u8) as usize;
            for j in 0..26 {
                if x != j {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    let mut sum = 1;
                    for k in 0..26 {
                        sum = (sum + dp[i - 1][k]) % MOD;
                    }
                    dp[i][j] = sum;
                }
            }
        }
        dp[n].iter().fold(0, |res, x| (res + x) % MOD)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let b = "aba".to_string();
    println!("res:{}", Solution::distinct_subseq_ii(b));
}
