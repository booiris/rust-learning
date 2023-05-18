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
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut now = 0;
        let mut itr1 = arr1.into_iter().rev();
        let mut itr2 = arr2.into_iter().rev();
        let mut res = vec![];
        loop {
            let (x, y) = (itr1.next(), itr2.next());
            if x.is_none() && y.is_none() {
                break;
            }
            let x = x.unwrap_or(0);
            let y = y.unwrap_or(0);
            let mut temp = x + y + now;
            if temp >= 2 {
                temp -= 2;
                now = -1;
            } else if temp < 0 {
                temp += 2;
                now = 1;
            } else {
                now = 0;
            }
            res.push(temp);
        }

        if now == 1 {
            res.push(1);
        } else if now == -1 {
            res.extend([1, 1]);
        }
        while let [_, .., 0] = res[..] {
            res.pop();
        }

        res.reverse();
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let arr1 = [1, 1, 1, 1, 1];
    let arr2 = [1, 0, 1];
    println!(
        "res:{:?}",
        Solution::add_negabinary(arr1.into(), arr2.into())
    );
}
