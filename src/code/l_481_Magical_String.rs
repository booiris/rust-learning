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
    pub fn magical_string(n: i32) -> i32 {
        let mut key: Vec<u8> = vec![];
        key.push(1);
        let mut i = 0;
        let mut j = 1;
        let mut flag = false;
        for _ in 0..n - 1 {
            match key[i] {
                1 => {
                    key.push(key[j - 1] ^ 3);
                    i += 1;
                    flag = false;
                }
                2 => {
                    if flag {
                        key.push(key[j - 1] ^ 3);
                        i += 1;
                        flag = false;
                    } else {
                        key.push(key[j - 1]);
                        flag = true;
                    }
                }
                _ => panic!(),
            }
            j += 1;
        }

        // println!("{:?}", key);

        key.iter().filter(|&&x| x == 1).count() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", Solution::magical_string(15));
}
