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
    pub fn num_different_integers(word: String) -> i32 {
        let mut key = HashSet::<String>::new();
        let mut w: Vec<char> = vec![];
        let mut zero = false;
        for x in word.chars() {
            if x.is_ascii_digit() {
                if w.len() == 0 && x == '0' {
                    zero = true;
                } else {
                    zero = false;
                    w.push(x);
                }
            } else if w.len() != 0 {
                let temp = w.iter().collect::<String>();
                key.insert(temp);
                w.clear();
            } else if zero {
                key.insert("0".to_owned());
                zero = false;
            }
        }
        if w.len() != 0 {
            let temp = w.iter().collect::<String>();
            key.insert(temp);
        } else if zero {
            key.insert("0".to_owned());
        }
        key.len() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "a1b01c001";
    println!("res:{}", Solution::num_different_integers(a.to_owned()));
}
