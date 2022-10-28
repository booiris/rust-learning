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
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut res: i64 = 0;
        let mut key: Vec<(i32, i32, i64)> = vec![];
        for (i, &x) in arr.iter().enumerate() {
            let mut pre = (0, -1, 0);
            while let Some(top) = key.last() {
                if x > top.0 {
                    pre = *top;
                    break;
                }
                key.pop();
            }
            let now = (pre.2 + (i as i32 - pre.1) as i64 * x as i64) % MOD;
            key.push((x, i as i32, now));
            // println!("{:?}", key);
            res = (res + now) % MOD;
        }
        res as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![11, 81, 94, 43, 3];
    println!("res:{}", Solution::sum_subarray_mins(a));
}
