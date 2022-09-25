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
    pub fn rotated_digits(n: i32) -> i32 {
        let mut res = 0;
        for mut num in 1..=n {
            let mut temp = vec![];
            let now = num;
            while num != 0 {
                let t = num % 10;
                match t {
                    0 | 1 | 8 => temp.push(t),
                    2 => temp.push(5),
                    5 => temp.push(2),
                    6 => temp.push(9),
                    9 => temp.push(6),
                    _ => {
                        temp.clear();
                        break;
                    }
                }
                num /= 10;
            }
            let mut key = 0;
            for i in (0..temp.len()).rev() {
                key *= 10;
                key += temp[i];
            }
            if key != now && key != 0 {
                res += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", Solution::rotated_digits(50));
}
