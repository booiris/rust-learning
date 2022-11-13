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
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut key = HashMap::<char, i32>::new();
        let mut now = 1;
        for x in order.chars() {
            key.insert(x, now);
            now += 1;
        }
        let s = s.into_bytes();
        let mut s = s.into_iter().map(|x| x as char).collect::<Vec<_>>();
        s.sort_unstable_by_key(|x| key.get(x).unwrap_or(&0));
        s.into_iter().collect::<String>()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
