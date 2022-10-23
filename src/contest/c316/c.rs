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
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut key = HashMap::new();
        for x in nums.iter().enumerate() {
            *key.entry(*x.1).or_insert(0) += cost[x.0] as i64;
        }
        let mut res = i64::MAX;

        let mut key = key.into_iter().collect::<Vec<_>>();
        key.sort_unstable();
        let mut k1 = vec![0; key.len() + 1];
        let mut cnt: i64 = 0;
        let mut pre = 0;
        for i in 1..=key.len() {
            k1[i] = k1[i - 1] + cnt * (key[i - 1].0 - pre) as i64;
            pre = key[i - 1].0;
            cnt += key[i - 1].1;
        }

        let mut k2 = vec![0; key.len() + 1];
        let mut cnt: i64 = 0;
        let mut pre = key[key.len() - 1].0;
        for i in (1..=key.len()).rev() {
            k2[i - 1] = k2[i] + cnt * (pre - key[i - 1].0) as i64;
            pre = key[i - 1].0;
            cnt += key[i - 1].1;
        }
        // println!("{:?}", key);
        // println!("{:?}", k1);
        // println!("{:?}", k2);

        for i in 0..key.len() {
            res = res.min(k1[i + 1] + k2[i]);
        }

        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![
        735103, 366367, 132236, 133334, 808160, 113001, 49051, 735598, 686615, 665317, 999793,
        426087, 587000, 649989, 509946, 743518,
    ];
    let b = vec![
        724182, 447415, 723725, 902336, 600863, 287644, 13836, 665183, 448859, 917248, 397790,
        898215, 790754, 320604, 468575, 825614,
    ];
    // let a = vec![1, 3, 5, 2];
    // let b = vec![2, 3, 1, 14];
    println!("res:{}", Solution::min_cost(a, b));
}
