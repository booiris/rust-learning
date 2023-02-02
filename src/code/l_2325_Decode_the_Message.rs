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
    pub fn decode_message(key: String, message: String) -> String {
        let mut mp = HashMap::new();
        let mut now = b'a';
        for c in key.chars() {
            if mp.get(&c).is_none() && c != ' ' {
                mp.insert(c, now);
                now += 1;
            }
        }
        message.chars().fold("".to_string(), |mut nows, x| {
            let a = match x {
                ' ' => ' ',
                _ => mp[&x] as char,
            };
            nows.push(a);
            nows
        })
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
