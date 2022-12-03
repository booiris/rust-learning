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
    pub fn second_highest(s: String) -> i32 {
        let mut a = s
            .chars()
            .into_iter()
            .filter(|x| x.is_ascii_digit())
            .collect::<HashSet<char>>()
            .into_iter()
            .collect::<Vec<_>>();
        if a.len() < 2 {
            return -1;
        }
        a.sort_unstable_by_key(|&x| Reverse(x));
        (a[1] as u8 - '0' as u8) as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
