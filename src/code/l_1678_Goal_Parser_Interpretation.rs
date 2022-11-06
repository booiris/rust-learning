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
    pub fn interpret(command: String) -> String {
        let mut res = vec![];
        let mut flag = 0;
        for x in command.chars() {
            match x {
                'G' => res.push(x),
                '(' => flag = 1,
                'a' => flag = 2,
                'l' => flag = 3,
                ')' => {
                    if flag == 1 {
                        res.push('o');
                    } else {
                        res.push('a');
                        res.push('l');
                    }
                }
                _ => panic!(),
            }
        }
        res.into_iter().collect()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
