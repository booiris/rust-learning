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

fn check(mat: &Vec<Vec<i32>>, nowx: usize, nowy: usize) -> bool {
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for x in 0..mat.len() {
        if mat[x][nowy] == 1 {
            cnt1 += 1;
        }
    }
    for x in 0..mat[0].len() {
        if mat[nowx][x] == 1 {
            cnt2 += 1;
        }
    }
    cnt1 == 1 && cnt2 == 1
}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 && check(&mat, i, j) {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
