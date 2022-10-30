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
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let key: usize = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => panic!(),
        };
        items.iter().filter(|x| x[key] == rule_value).count() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
