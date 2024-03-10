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
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        fruits
            .iter()
            .skip(1)
            .fold(
                (1, 1, 1, fruits[0], fruits[0]),
                |(ans, cnt, c_cnt, last_fruit, other_fruit), &fruit| match (
                    fruit == last_fruit,
                    fruit == other_fruit,
                ) {
                    (true, _) => (
                        ans.max(cnt + 1),
                        cnt + 1,
                        c_cnt + 1,
                        last_fruit,
                        other_fruit,
                    ),
                    (false, true) => (ans.max(cnt + 1), cnt + 1, 1, other_fruit, last_fruit),
                    (false, false) => (ans.max(c_cnt + 1), c_cnt + 1, 1, fruit, last_fruit),
                },
            )
            .0
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
