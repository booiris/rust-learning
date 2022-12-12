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
    pub fn beauty_sum(s: String) -> i32 {
        let mut key = HashMap::<u8, i32>::new();
        let s = s.as_bytes();
        let mut res = 0;
        for i in 0..s.len() {
            key.clear();
            for j in i..s.len() {
                *key.entry(s[j]).or_default() += 1;
                let minn = key.iter().min_by_key(|x| x.1).unwrap().1;
                let maxn = key.iter().max_by_key(|x| x.1).unwrap().1;
                res += maxn - minn;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "aabcb";
    println!("res:{}", Solution::beauty_sum(a.into()));
}
