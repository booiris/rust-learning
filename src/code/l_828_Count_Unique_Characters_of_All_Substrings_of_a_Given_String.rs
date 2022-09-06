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
    pub fn unique_letter_string(s: String) -> i32 {
        let mut key = vec![[-1, s.len() as i32]; s.len()];
        for c in 'A'..='Z' {
            let mut pre: i32 = -1;
            for (index, x) in s.chars().enumerate() {
                if x == c {
                    key[index][0] = pre;
                    if pre != -1 {
                        key[pre as usize][1] = index as i32;
                    }
                    pre = index as i32;
                }
            }
        }
        let mut res = 0;
        for (index, _) in s.chars().enumerate() {
            res += (index as i32 - key[index][0]) * (key[index][1] - index as i32);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "IECIYJSQHMDHQPCOTCQTVYEQMEYGGVPBUPKVHAAGBQKAQQVMWTMZZSEGTYWTBCN";
    println!("res:{}", Solution::unique_letter_string(a.to_string()));
}
