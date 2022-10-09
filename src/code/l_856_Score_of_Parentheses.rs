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
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut key = vec![];
        for x in s.chars() {
            if x == '(' {
                key.push(-1);
            } else {
                let mut sum = 0;
                while let Some(x) = key.pop() {
                    if x == -1 {
                        break;
                    }
                    sum += x;
                }
                if sum == 0 {
                    key.push(1);
                } else {
                    key.push(sum * 2);
                }
            }
        }
        key[0]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
