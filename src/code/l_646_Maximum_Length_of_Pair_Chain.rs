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
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let mut key = vec![];
        pairs.sort_unstable();
        for x in pairs {
            let (s, e) = (x[0], x[1]);
            let index = key.partition_point(|x| x < &s);
            let index = key.get_mut(index);
            match index {
                Some(x) => *x = min(*x, e),
                _ => key.push(e),
            }
        }
        key.len() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let temp = vec![vec![1, 2], vec![7, 8], vec![4, 5]];
    println!("res:{}", Solution::find_longest_chain(temp));
}
