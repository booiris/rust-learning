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
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut key = vec![0; n as usize];
        let mut pre = 0;
        for x in logs {
            key[x[0] as usize] += x[1] - pre;
            pre = x[1];
        }
        key.iter()
            .enumerate()
            .fold(
                (0, key[0]),
                |(res, now), (i, x)| {
                    if x < &now {
                        (i, *x)
                    } else {
                        (res, now)
                    }
                },
            )
            .0 as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
