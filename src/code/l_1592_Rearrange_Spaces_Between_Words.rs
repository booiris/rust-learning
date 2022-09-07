#![allow(dead_code, unused_imports, unused_macros)]
use std::borrow::Borrow;
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
    pub fn reorder_spaces(text: String) -> String {
        let mut words = text.split_whitespace();
        let cnt = text.chars().filter(|&x| x == ' ').count();
        let key = text.split_whitespace().count();
        let mut res = "".to_string();
        if key == 1 {
            res += words.next().unwrap();
            for _ in 0..cnt {
                res.push(' ');
            }
            return res;
        }
        for x in words {
            res += x;
            for _ in 0..cnt / (key - 1) {
                res.push(' ');
            }
        }
        for _ in 0..cnt / (key - 1) {
            res.pop();
        }
        for _ in 0..cnt % (key - 1) {
            res.push(' ');
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!(
        "res:{}",
        Solution::reorder_spaces("A     dddd asdasd".to_string())
    );
}
