#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
use std::slice::Iter;
use std::str::Chars;
#[cfg(feature = "local")]
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[inline]
fn check(query: String, pattern: &[char]) -> bool {
    let mut index = 0;
    for c in query.chars() {
        if index < pattern.len() && pattern[index] == c {
            index += 1;
        } else if c.is_ascii_uppercase() {
            return false;
        }
    }
    index == pattern.len()
}

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.chars().collect::<Vec<_>>();
        let mut res = vec![];
        for q in queries {
            res.push(check(q, &pattern));
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let q = [
        "FooBar",
        "FooBarTest",
        "FootBall",
        "FrameBuffer",
        "ForceFeedBack",
    ];
    let p = "FoBaT";
    println!(
        "res:{:?}",
        Solution::camel_match(q.iter().map(|x| x.to_string()).collect(), p.into())
    );
}
