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
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut key = HashMap::<i32, HashSet<i32>>::new();
        for x in logs {
            key.entry(x[0]).or_default().insert(x[1]);
        }
        let mut res = vec![0; k as usize];
        for (_, freq) in key {
            if freq.len() > 0 && freq.len() <= k as usize {
                res[freq.len() - 1] += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
