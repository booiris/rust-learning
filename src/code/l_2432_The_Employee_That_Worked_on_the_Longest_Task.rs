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
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut maxn = 0;
        let mut pre = 0;
        for x in logs {
            if maxn < x[1] - pre {
                maxn = x[1] - pre;
                res = x[0];
            } else if maxn == x[1] - pre {
                res = res.min(x[0]);
            }
            pre = x[1];
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let logs = [[0, 3], [2, 5], [0, 9], [1, 15]];
    let logs = logs.iter().map(|&x| x.into()).collect::<Vec<_>>();
    println!("res:{}", Solution::hardest_worker(10, logs));
}
