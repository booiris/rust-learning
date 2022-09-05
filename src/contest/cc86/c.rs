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

fn check(key: &[i32; 15], mat: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for i in 0..mat.len() {
        let mut flag = true;
        for j in 0..mat[0].len() {
            if mat[i][j] == 1 && key[j] != 1 {
                flag = false;
            }
        }
        if flag {
            res += 1;
        }
    }
    res
}

impl Solution {
    pub fn maximum_rows(mat: Vec<Vec<i32>>, cols: i32) -> i32 {
        let (n, m) = (mat.len(), mat[0].len());
        let mut res = 0;
        for k in 0..1 << m {
            let mut temp = k;
            let mut index = 0;
            let mut key = [0; 15];
            let mut cnt = 0;
            while temp != 0 {
                if temp & 1 == 1 {
                    key[index] = 1;
                    cnt += 1;
                }
                index += 1;
                temp >>= 1;
            }
            if cnt != cols {
                continue;
            }
            res = max(res, check(&key, &mat));
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
