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

#[derive(PartialEq, Debug)]
enum State {
    A,
    B,
    C,
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut key = Vec::<State>::new();
        for x in s.chars() {
            match x {
                'a' => key.push(State::A),
                'b' => {
                    if let Some(top) = key.pop() {
                        if top != State::A {
                            return false;
                        } else {
                            key.push(State::B);
                        }
                    } else {
                        return false;
                    }
                }
                'c' => {
                    if let Some(top) = key.pop() {
                        if top != State::B {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => panic!(),
            };
        }
        key.is_empty()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "aabcbc";
    println!("res:{}", Solution::is_valid(a.into()));
}
