#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::*;

struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[cfg(test)]
pub fn main() {
    println!("res:");
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let key: HashMap<char, i32> =
            hashmap!['I'=>1,'V'=>5,'X'=>10,'L'=>50,'C'=>100,'D'=>500,'M'=>1000];
        panic!()
    }
}
