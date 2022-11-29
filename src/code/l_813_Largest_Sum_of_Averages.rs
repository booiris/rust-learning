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
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut key: Vec<Vec<f64>> = vec![vec![-1.0; k as usize + 1]; nums.len() + 1];
        key[0][0] = 0.0;
        for i in 1..=nums.len() {
            let mut sum = 0;
            for j in (1..=i).rev() {
                sum += nums[j - 1];
                for kk in 0..k {
                    let kk = kk as usize;
                    if key[j - 1][kk] < 0.0 {
                        continue;
                    }
                    let temp = sum as f64 / (i - j + 1) as f64;
                    key[i][kk + 1] = key[i][kk + 1].max(key[j - 1][kk] + temp);
                }
            }
            // println!("{:?}", key[i]);
        }
        key[nums.len()].iter().copied().fold(0.0, f64::max)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 2, 3, 4, 5, 6, 7];
    println!("res:{}", Solution::largest_sum_of_averages(a, 4));
}
