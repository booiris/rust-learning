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

struct LUPrefix {
    v: Vec<bool>,
    now: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {
    fn new(n: i32) -> Self {
        LUPrefix {
            v: vec![false; n as usize],
            now: 0,
        }
    }

    fn upload(&mut self, video: i32) {
        self.v[(video - 1) as usize] = true;
    }

    fn longest(&mut self) -> i32 {
        while self.now < self.v.len() as i32 && self.v[self.now as usize] {
            self.now += 1;
        }
        self.now
    }
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * let obj = LUPrefix::new(n);
 * obj.upload(video);
 * let ret_2: i32 = obj.longest();
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
