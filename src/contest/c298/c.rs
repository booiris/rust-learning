#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        
    }
}
#[cfg(test)]
pub fn main() {
    println!("res:");
}


