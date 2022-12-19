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

struct MaxQueue {
    key: VecDeque<i32>,
    maxn: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxQueue {
    fn new() -> Self {
        MaxQueue {
            key: VecDeque::new(),
            maxn: VecDeque::new(),
        }
    }

    fn max_value(&self) -> i32 {
        *self.maxn.front().unwrap_or(&-1)
    }

    fn push_back(&mut self, value: i32) {
        self.key.push_back(value);
        while let Some(&max) = self.maxn.back() {
            if max < value {
                self.maxn.pop_back();
            }else{
                break;
            }
        }
        self.maxn.push_back(value);
    }

    fn pop_front(&mut self) -> i32 {
        let res = self.key.pop_front().unwrap_or(-1);
        if &res == self.maxn.front().unwrap_or(&-2){
            self.maxn.pop_front();
        }
        res
    }
}

/**
 * Your MaxQueue object will be instantiated and called as such:
 * let obj = MaxQueue::new();
 * let ret_1: i32 = obj.max_value();
 * obj.push_back(value);
 * let ret_3: i32 = obj.pop_front();
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
