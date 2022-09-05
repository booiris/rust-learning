#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
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
    pub fn count_asterisks(s: String) -> i32 {
        let mut flag = false;
        let mut res = 0;
        for c in s.chars() {
            if !flag && c == '*' {
                res += 1;
            }
            if c == '|' {
                flag = !flag;
            }
        }
        res
    }
}

#[cfg(test)]
pub fn main() {
    println!("res:");
}
