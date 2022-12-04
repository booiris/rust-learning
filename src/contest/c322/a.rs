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
    pub fn is_circular_sentence(sentence: String) -> bool {
        let a = sentence.split_ascii_whitespace();
        let mut pre = '#';
        let mut f = '#';
        for x in a {
            let c = x.chars().last().unwrap();
            let t = x.chars().nth(0).unwrap();
            if pre == '#' {
                f = t;
            } else if pre != t {
                return false;
            }
            pre = c;
        }
        f == pre
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
