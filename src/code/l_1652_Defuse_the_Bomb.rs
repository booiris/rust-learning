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
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![0; code.len()];
        if k == 0 {
            return res;
        }
        let len = res.len();
        for (i, _) in code.iter().enumerate() {
            let s;
            let e;
            if k > 0 {
                s = 1;
                e = k;
            } else {
                s = k;
                e = -1;
            }
            let mut sum = 0;
            for j in s..=e {
                let now = i as i32 + j;
                let now = (now as usize + len) % len;
                sum += code[now];
            }
            res[i] = sum;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![5, 7, 1, 4];
    println!("res:{:?}", Solution::decrypt(a, 3));
}
