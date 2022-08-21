#[allow(dead_code)]
#[allow(unused_imports)]
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
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (index, word) in sentence.split_whitespace().enumerate() {
            if word.starts_with(&search_word) {
                return index as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "i love eating burger".to_string();
    let b = "burg".to_string();
    println!("res:{}", Solution::is_prefix_of_word(a, b));
}
