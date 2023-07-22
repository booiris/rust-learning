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
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        let mut cnt3 = 0;
        for x in bills {
            match x {
                5 => {
                    cnt1 += 1;
                }
                10 => {
                    if cnt1 == 0 {
                        return false;
                    }
                    cnt1 -= 1;
                    cnt2 += 1;
                }
                20 => {
                    if cnt2 == 0 {
                        if cnt1 < 3 {
                            return false;
                        }
                        cnt1 -= 3;
                        cnt3 += 1;
                    } else {
                        cnt2 -= 1;
                        if cnt1 == 0 {
                            return false;
                        }
                        cnt1 -= 1;
                        cnt3 += 1;
                    }
                }
                _ => panic!(),
            }
        }
        true
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
