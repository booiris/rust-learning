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
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut key = HashMap::<i32, i32>::new();
        let mut cnt = HashMap::<i32, i32>::new();
        let mut res = 0;
        let mut index = 0;
        let mut flag = false;
        for x in nums {
            let now;
            if let Some(val) = cnt.get_mut(&x) {
                key.insert(*val, key[val] - 1);
                *val += 1;
                let v = key.entry(*val).or_insert(0);
                *v += 1;
                now = *val;
            } else {
                cnt.insert(x, 1);
                let v = key.entry(1).or_insert(0);
                *v += 1;
                now = 1;
            }
            index += 1;
            let temp = key[&now] * now;
            if temp == index - 1
                || key.get(&(index - temp)).is_some()
                || (key[&now] == 1 && temp == index)
            {
                res = index;
            }
            if flag {
                res = index;
            }
            flag = false;
            if temp == index {
                flag = true;
            }
            // println!("key=={:?}", key);
            // println!("cnt=={:?}", cnt);
            // println!("res=={:?}", res);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let ini = vec![2, 2, 1, 1, 5, 3, 3, 5];
    println!("res:{}", Solution::max_equal_freq(ini));
}
