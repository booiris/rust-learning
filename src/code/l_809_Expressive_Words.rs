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

fn conver(s: String) -> Vec<(char, i32)> {
    let mut key: Vec<(char, i32)> = vec![];
    let mut pre = '#';
    for x in s.chars() {
        if pre == x {
            if let Some(last) = key.last_mut() {
                last.1 += 1;
            }
        } else {
            key.push((x, 1));
        }
        pre = x;
    }
    key
}

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let key = conver(s);
        let mut res = 0;
        for s in words {
            let now = conver(s);
            if key.len() != now.len() {
                continue;
            }
            let mut flag = true;
            for (x, y) in key.iter().zip(now.iter()) {
                if x.0 != y.0 || (x.1 < 3 && x.1 != y.1) || x.1 < y.1 {
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
    //     "dddiiiinnssssssoooo"
    // ["dinnssoo","ddinso","ddiinnso","ddiinnssoo","ddiinso","dinsoo","ddiinsso","dinssoo","dinso"]
    let s = "dddiiiinnssssssoooo";
    let a = vec![
        "dinnssoo",
        "ddinso",
        "ddiinnso",
        "ddiinnssoo",
        "ddiinso",
        "dinsoo",
        "ddiinsso",
        "dinssoo",
        "dinso",
    ];
    let a = a.into_iter().map(|x| x.to_owned()).collect::<Vec<_>>();
    println!("res:{}", Solution::expressive_words(s.to_owned(), a));
}
