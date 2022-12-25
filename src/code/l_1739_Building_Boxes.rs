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
    pub fn minimum_boxes(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let n = n as i64;
        let mut now: i64 = 1;
        let mut res = 1;
        let mut index = 1;
        while now < n {
            index += 1;
            if now + (index + 1) * index / 2 <= n {
                res += index;
                now += (index + 1) * index / 2;
            } else {
                break;
            }
        }
        let key = n - now;
        if key == 0 {
            return res as i32;
        }
        res += 1;
        now = 1;
        index = 1;
        while now < key {
            index += 1;
            res += 1;
            now += index;
        }
        res as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", Solution::minimum_boxes(8));
}
