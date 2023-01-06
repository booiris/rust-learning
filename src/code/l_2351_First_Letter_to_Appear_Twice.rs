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
    pub fn repeated_character(s: String) -> char {
        let mut key = HashSet::new();
        for x in s.chars(){
            if key.get(&x).is_some(){
                return x;
            }
            key.insert(x);
        }
        panic!()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}


