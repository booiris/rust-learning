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

fn dfs(s: &Vec<char>, depth: usize, res: &mut Vec<String>, now: &mut Vec<char>) {
    if depth == s.len() {
        let temp = now.iter().collect::<String>();
        res.push(temp.clone());
        return;
    }
    if s[depth].is_alphabetic() {
        now.push(s[depth]);
        dfs(s, depth + 1, res, now);
        now.pop();
        now.push((s[depth] as u8 ^ 32) as char);
        dfs(s, depth + 1, res, now);
        now.pop();
    } else {
        now.push(s[depth]);
        dfs(s, depth + 1, res, now);
        now.pop();
    }
}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let s = s.chars().into_iter().collect::<Vec<_>>();
        let mut res = vec![];
        let mut now = vec![];
        dfs(&s, 0, &mut res, &mut now);
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
