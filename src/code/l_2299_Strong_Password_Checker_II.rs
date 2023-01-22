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
    pub fn strong_password_checker_ii(password: String) -> bool {
        let sp = "!@#$%^&*()-+";
        let password = password.chars().collect::<Vec<_>>();
        password.len() >= 8
            && password.iter().any(|x| x.is_ascii_lowercase())
            && password.iter().any(|x| x.is_ascii_uppercase())
            && password.iter().any(|x| x.is_ascii_digit())
            && password.windows(2).all(|x| x[0] != x[1])
            && password.iter().any(|x| sp.contains(*x))
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
