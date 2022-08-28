#[allow(dead_code)]
#[allow(unused_imports)]
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
    pub fn preimage_size_fzf(k: i32) -> i32 {
        0
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let mut now1 = 0;
    let mut now2 = 0;
    for i in 0..1e5 as i32{
        
    }
    println!("res:");
}
