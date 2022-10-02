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
    pub fn can_transform(start: String, end: String) -> bool {
        let mut index1 = 0;
        let mut index2 = 0;
        let n = start.len();
        let mut start = start.chars().into_iter().collect::<Vec<_>>();
        let mut end = end.chars().into_iter().collect::<Vec<_>>();
        loop {
            while index1 < n && start[index1] == 'X' {
                index1 += 1;
            }
            while index2 < n && end[index2] == 'X' {
                index2 += 1;
            }

            if index1 >= n && index2 >= n {
                return true;
            }

            if index1 >= n || index2 >= n || start[index1] != end[index2] {
                return false;
            }
            if (index1 < index2 && start[index1] == 'L')
                || (index1 > index2 && start[index1] == 'R')
            {
                return false;
            }

            index1 += 1;
            index2 += 1;
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
