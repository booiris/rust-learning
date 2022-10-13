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
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut key = HashSet::new();
        let mut max = 1;
        let mut now = 0;
        for x in arr {
            max = max.max(x + 1 - now);
            key.insert(x);
            // println!("{} {} {}", key.len(), max, now);
            if key.len() as i32 == max {
                key.clear();
                res += 1;
                now += max;
                max = 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 0, 2, 3, 4];
    println!("res:{}", Solution::max_chunks_to_sorted(a));
}
