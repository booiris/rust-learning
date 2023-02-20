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
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let k = suits[0];
        if suits.iter().all(|x| x == &k) {
            "Flush".into()
        } else {
            let ranks = ranks.into_iter().fold(HashMap::new(), |mut now, x| {
                *now.entry(x).or_insert(0) += 1;
                now
            });
            let maxn = *ranks.iter().max_by_key(|x| x.1).unwrap().1;
            match maxn {
                3 => "Three of a Kind".into(),
                2 => "Pair".into(),
                1 => "High Card".into(),
                _ => "Three of a Kind".into(),
            }
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
