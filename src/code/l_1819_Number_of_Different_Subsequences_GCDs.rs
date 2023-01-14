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

#[inline]
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let maxn = *nums.iter().max().unwrap() as usize;
        let mut key = vec![false; maxn + 5];
        let mut res = 0;
        for x in nums {
            key[x as usize] = true;
        }
        for i in 1..=maxn {
            let mut now = 0;
            for j in (i..=maxn).step_by(i) {
                if key[j] {
                    now = gcd(now, j as i32);
                }
                if now as usize == i {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![6, 10, 3];
    println!("res:{}", Solution::count_different_subsequence_gc_ds(a));
}
