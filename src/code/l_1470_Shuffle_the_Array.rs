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
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        nums.iter()
            .zip(nums.iter().skip(n as usize))
            .flat_map(|(&x, &y)| [x, y])
            .collect()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
