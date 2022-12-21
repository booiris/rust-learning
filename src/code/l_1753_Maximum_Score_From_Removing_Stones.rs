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

fn get(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    let mut res = 0;
    while c > 0 && (a > 0 || b > 0) {
        if a > b {
            a -= 1;
        } else {
            b -= 1;
        }
        c -= 1;
        res += 1;
    }
    res + min(a, b)
}

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        get(a, b, c).max(get(c, a, b)).max(get(b, c, a))
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
