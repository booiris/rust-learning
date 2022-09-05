#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut key = [false; 256];
        for c in s.chars() {
            key[c as usize] = true;
        }
        for res in ('A'..='Z').rev() {
            if key[res as usize] && key[res as usize + 32] {
                return res.to_string();
            }
        }
        return "".to_string();
    }
}

#[cfg(test)]
pub fn main() {
    println!("res:{}", Solution::greatest_letter("nzmguNAEtJHkQaWDVSKxRCUivXpGLBcsjeobYPFwTZqrhlyOIfdM".to_string()));
}
