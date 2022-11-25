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
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut key: HashMap<i32, i32> = HashMap::new();
        for x in low_limit..=high_limit {
            let mut x = x;
            let mut now = 0;
            while x != 0 {
                now += x % 10;
                x /= 10;
            }
            *key.entry(now).or_default() += 1;
        }
        key.into_iter().max_by_key(|x| x.1).unwrap().1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
