#[allow(dead_code)]
#[allow(unused_imports)]
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
    pub fn max_score(s: String) -> i32 {
        let mut key1 = vec![0; s.len() + 1];
        let mut key2 = vec![0; s.len() + 1];
        for i in 1..=s.len() {
            key1[i] = key1[i - 1];
            if let Some(x) = s.chars().nth(i - 1) {
                if x == '0' {
                    key1[i] += 1;
                }
            }
        }
        for i in (0..s.len()).rev() {
            key2[i] = key2[i + 1];
            if let Some(x) = s.chars().nth(i) {
                if x == '1' {
                    key2[i] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 1..s.len() {
            res = max(res, key1[i] + key2[i]);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
