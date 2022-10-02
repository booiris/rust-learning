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
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let mut rows = vec![-1; matrix.len()];
        let mut cols = vec![-1; matrix[0].len()];
        for r in 0..rows.len() {
            for c in 0..cols.len() {
                if matrix[r][c] == 0 {
                    rows[r] = 0;
                    cols[c] = 0;
                }
            }
        }

        for r in 0..rows.len() {
            for c in 0..cols.len() {
                matrix[r][c] &= rows[r] & cols[c];
            }
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
