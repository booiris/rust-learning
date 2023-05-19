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
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut c = vec![vec![0; 8]; 8];
        for i in 0..8 {
            c[i][0] = 1;
            c[i][i] = 1;
            for j in 1..i {
                c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
            }
        }
        let mut cnt = HashMap::new();
        for x in tiles.chars() {
            *cnt.entry(x).or_insert(0) += 1;
        }
        let mut dp = vec![vec![0; 8]; 8];
        let n = tiles.len();
        let cnt_len = cnt.len();
        dp[0][0] = 1;
        for (i, (_, v)) in cnt.into_iter().enumerate() {
            let i = i + 1;
            for j in 0..=n {
                for k in 0..=j.min(v as usize) {
                    dp[i][j] += dp[i - 1][j - k] * c[j][k];
                }
            }
        }
        dp[cnt_len].iter().skip(1).sum()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let x = "AAB";
    println!("res:{}", Solution::num_tile_possibilities(x.into()));
}
