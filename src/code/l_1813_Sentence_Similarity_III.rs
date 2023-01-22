#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::mem::swap;
use std::ops::Bound::*;
use std::str::SplitAsciiWhitespace;
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
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let sentence1 = sentence1
            .split_ascii_whitespace()
            .into_iter()
            .collect::<Vec<_>>();
        let sentence2 = sentence2
            .split_ascii_whitespace()
            .into_iter()
            .collect::<Vec<_>>();
        let i = sentence1
            .iter()
            .zip(sentence2.iter())
            .position(|(x, y)| x != y);
        let j = sentence1
            .iter()
            .rev()
            .zip(sentence2.iter().rev())
            .position(|(x, y)| x != y);
        let aim = min(sentence1.len(), sentence2.len());
        i.unwrap_or(aim) + j.unwrap_or(aim) >= aim
    }
}

#[cfg(feature = "local")]
pub fn main() {
    // "of"
    // "A lot of words"
    let a = "A";
    let b = "a A b A";
    println!(
        "res:{}",
        Solution::are_sentences_similar(a.into(), b.into())
    );
}
