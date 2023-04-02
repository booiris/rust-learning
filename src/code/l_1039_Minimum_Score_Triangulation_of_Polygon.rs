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
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; n];
        for i in 0..n {
            dp[i][2] = 0;
        }

        for c in 3..=n {
            for i in 0..n {
                for j in (i + 1)..(i + c - 1) {
                    let jn = j % n;
                    dp[i][c] = dp[i][c].min(
                        values[i] * values[jn] * values[(i + c - 1) % n]
                            + dp[i][j + 1 - i]
                            + dp[jn][i + c - j],
                    );
                }
            }
        }
        dp[0][n]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let values = [1, 3, 1, 4, 1, 5];
    println!("res:{}", Solution::min_score_triangulation(values.into()));
}
