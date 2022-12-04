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
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut key = vec![];
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let sum = skill.iter().sum::<i32>();
        if sum % (skill.len() as i32 / 2) != 0 {
            return -1;
        }
        let aim = sum / (skill.len() as i32 / 2);
        for x in &skill {
            *mp.entry(*x).or_default() += 1;
        }
        for x in skill {
            if let Some(y) = mp.get(&x) {
                if *y == 0 {
                    continue;
                }
            }
            *mp.entry(x).or_default() -= 1;
            if let Some(y) = mp.get_mut(&(aim - x)) {
                if *y == 0{
                    return -1;
                }
                *y -= 1;
                key.push((x, aim - x));
            } else {
                return -1;
            }
        }
        key.into_iter().map(|x| x.0 as i64 * x.1 as i64).sum()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 1, 1, 2, 3, 3, 3, 7, 7, 8, 8, 8, 9, 9];
    println!("res:{}", Solution::divide_players(a));
}
