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

fn check(aim: i64, max_sum: i64, index: i64, n: i64) -> bool {
    let mut sum = 0;
    let mut key = |aim: i64, index: i64| {
        if aim - index as i64 > 0 {
            sum += (aim + aim - index) * (index + 1) / 2;
        } else {
            sum += (aim + 1) * (aim) / 2;
            sum += index - aim + 1;
        }
    };
    key(aim, index);
    key(aim, n - index - 1);
    sum - aim <= max_sum
}

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut low = 1;
        let mut high = 1e9 as i64;
        while low <= high {
            let mid = low + (high - low) / 2;
            if check(mid, max_sum as i64, index as i64, n as i64) {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        high as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", Solution::max_value(4, 0, 4));
}
