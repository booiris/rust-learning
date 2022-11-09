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

fn check(now: &[char], p: usize) -> bool {
    if p > 1 && now[0] == '0' {
        return false;
    }
    if now.len() - p > 0 && now[now.len() - 1] == '0' {
        return false;
    }
    true
}

fn get(now: &[char]) -> Vec<String> {
    let mut res = vec![];
    for p in 1..=now.len() {
        if check(now, p) {
            let mut temp = now[0..p].iter().collect::<String>();
            if p != now.len() {
                temp += ".";
                temp += &now[p..now.len()].iter().collect::<String>();
            }
            res.push(temp);
        }
    }
    res
}

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut res = vec![];
        let s = s.chars().collect::<Vec<_>>();
        for i in 2..s.len() - 1 {
            let k1 = get(&s[1..i]);
            let k2 = get(&s[i..s.len() - 1]);
            if k1.len() != 0 && k2.len() != 0 {
                for x in &k1 {
                    let now = "(".to_string() + x;
                    for y in &k2 {
                        let mut temp = now.clone();
                        temp += ", ";
                        temp += y;
                        temp += ")";
                        res.push(temp);
                    }
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "(00011)";
    println!("res:{:?}", Solution::ambiguous_coordinates(a.to_owned()));
}
