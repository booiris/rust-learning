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
    pub fn delete_string(s: String) -> i32 {
        let n = s.len();
        let mut res = 1;
        let mut i = n - 1;
        while i > 0 {
            let mut flag = false;
            for k in 0..n - i {
                if &s[i..(i + k + 1).min(n)] == &s[i.checked_sub(k + 1).unwrap_or(0)..i] {
                    res += 1;
                    i -= k + 1;
                    flag = true;
                    break;
                }
            }
            if !flag {
                i -= 1;
                res = 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res: {}", Solution::delete_string("abcabcdabc".to_string()));
}
