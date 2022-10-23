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
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut word1 = word1.chars().into_iter().collect::<Vec<_>>();
        word1.reverse();
        let mut word2 = word2.chars().into_iter().collect::<Vec<_>>();
        word2.reverse();
        let mut res = "".to_string();
        while !word1.is_empty() || !word2.is_empty() {
            if let Some(x) = word1.pop() {
                res += &x.to_string();
            }
            if let Some(x) = word2.pop() {
                res += &x.to_string();
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "abc";
    let b = "pqr";
    println!(
        "res:{}",
        Solution::merge_alternately(a.to_owned(), b.to_owned())
    );
}
