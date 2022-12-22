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
    return gcd(b, a % b);
}

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut key = HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let temp = gcd(nums[i], nums[j]);
                key.insert((nums[i], nums[j]), temp);
            }
        }
        let mut dp = vec![HashMap::<usize, i32>::new(); 2];
        dp[0].insert(0, 0);
        let maxn = (1 << nums.len()) as usize;
        for x in 0..nums.len() / 2 {
            for k in 0..maxn {
                if let Some(&now) = dp[0].get(&k) {
                    for i in 0..nums.len() {
                        for j in i + 1..nums.len() {
                            if (k & (1 << i) != 0) || (k & (1 << j) != 0) {
                                continue;
                            }
                            let now = *key.get(&(nums[i], nums[j])).unwrap() * (1 + x as i32) + now;
                            let k = k | (1 << i) | (1 << j);
                            let a = dp[1].entry(k).or_insert(0);
                            *a = now.max(*a);
                        }
                    }
                }
            }
            dp.swap(0, 1);
        }
        *dp[0].iter().max_by_key(|x| x.1).unwrap().1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
