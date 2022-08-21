#![allow(unused_imports)]
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
    pub fn largest_palindromic(num: String) -> String {
        let mut key = [0; 10];
        for x in num.chars() {
            key[x as usize - '0' as usize] += 1;
        }
        let mut mid: char = '#';
        let mut res = String::from("");
        for i in 1..10_usize {
            if key[i] == 0 {
                continue;
            }
            for k in 0..key[i] / 2 {
                res = String::from((i as u8 + '0' as u8) as char)
                    + &res
                    + &String::from((i as u8 + '0' as u8) as char);
            }
            if key[i] % 2 == 1 {
                mid = (i as u8 + '0' as u8) as char;
            }
        }
        if res == "" {
            if mid == '#' {
                return String::from("0");
            } else {
                return String::from(mid);
            }
        }
        let mut temp = String::from("");
        if key[0] % 2 == 1 {
            key[0] -= 1;
            if mid == '#' {
                mid = '0';
            }
        }
        for _ in 0..key[0] {
            temp += "0";
        }
        res = String::from(&res[0..res.len() / 2]) + &temp + &res[res.len() / 2..res.len()];
        if mid != '#' {
            res = String::from(&res[0..res.len() / 2])
                + &mid.to_string()
                + &res[res.len() / 2..res.len()];
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
