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

struct StockSpanner {
    key: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        StockSpanner { key: vec![] }
    }

    fn next(&self, price: i32) -> i32 {
        let mut now = 1;
        while let Some(x) = self.key.last() {
            if x.0 > price {
                break;
            }
            now += x.1;
            self.key.pop();
        }
        self.key.push((price, now));
        now
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
