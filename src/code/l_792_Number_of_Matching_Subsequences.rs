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
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut key = vec![[0; 26]; s.len() + 1];
        let mut now = [i32::MAX; 26];
        let s: Vec<char> = s.chars().collect();
        for i in (0..s.len()).rev() {
            let index = (s[i] as u8 - 'a' as u8) as usize;
            key[i + 1] = now;
            now[index] = now[index].min(i as i32 + 1);
        }
        key[0] = now;
        let mut res = 0;
        for x in words {
            let mut flag = true;
            let mut now = 0;
            for y in x.chars() {
                let index = (y as u8 - 'a' as u8) as usize;
                now = key[now as usize][index];
                if now == i32::MAX {
                    flag = false;
                    break;
                }
            }
            if flag {
                res += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let s = "dsahjpjauf";
    let b = vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"];
    let b = b.into_iter().map(|x| x.to_owned()).collect();
    println!("res:{}", Solution::num_matching_subseq(s.to_owned(), b));
}
