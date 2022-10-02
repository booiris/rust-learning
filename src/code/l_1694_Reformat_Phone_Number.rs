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
    pub fn reformat_number(number: String) -> String {
        enum State {
            S0,
            S1(char),
            S2(char, char),
            S3(char, char, char),
            S4(char, char, char, char),
        }
        use State::*;

        let mut state = S0;
        let mut res = String::new();

        for c in number.chars() {
            if c == '-' || c == ' ' {
                continue;
            }

            state = match state {
                S0 => S1(c),
                S1(c1) => S2(c1, c),
                S2(c1, c2) => S3(c1, c2, c),
                S3(c1, c2, c3) => S4(c1, c2, c3, c),
                S4(c1, c2, c3, c4) => {
                    res.push_str(&format!("{c1}{c2}{c3}-"));
                    S2(c4, c)
                }
            }
        }

        res.push_str(
            &(match state {
                S0 => "".to_string(),
                S1(c1) => format!("{c1}"),
                S2(c1, c2) => format!("{c1}{c2}"),
                S3(c1, c2, c3) => format!("{c1}{c2}{c3}"),
                S4(c1, c2, c3, c4) => format!("{c1}{c2}-{c3}{c4}"),
            }),
        );

        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
