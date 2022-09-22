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
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut key: HashMap<i32, usize> = HashMap::new();
        arr.iter().enumerate().for_each(|(i, &x)| {
            key.insert(x, i);
        });
        for x in pieces {
            let mut pre = -1;
            for y in x {
                if pre != -1 {
                    if let (Some(z1), Some(z2)) = (key.get(&pre), key.get(&y)) {
                        if z1 + 1 != *z2 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if key.get(&y).is_none() {
                    return false;
                }
                pre = y;
            }
        }
        true
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
