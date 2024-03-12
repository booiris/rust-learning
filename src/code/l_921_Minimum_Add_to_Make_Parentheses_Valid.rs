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
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut key = vec![];
        let mut res = 0;
        for x in s {
            if x == '(' {
                key.push(x);
            } else if let Some(x) = key.last() {
                key.pop();
            } else {
                res += 1;
            }
        }
        res += key.len() as i32;
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
