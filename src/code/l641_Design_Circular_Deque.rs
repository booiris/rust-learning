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

struct MyCircularDeque {
    data: VecDeque<i32>,
    maxsize: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            data: VecDeque::with_capacity(k as usize),
            maxsize: k as usize,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.data.len() < self.maxsize {
            self.data.push_front(value);
            true
        } else {
            false
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.data.len() < self.maxsize {
            self.data.push_back(value);
            true
        } else {
            false
        }
    }

    fn delete_front(&mut self) -> bool {
        if self.data.len() > 0 {
            self.data.pop_front();
            true
        } else {
            false
        }
    }

    fn delete_last(&mut self) -> bool {
        if self.data.len() > 0 {
            self.data.pop_back();
            true
        } else {
            false
        }
    }

    fn get_front(&self) -> i32 {
        if self.data.is_empty() {
            -1
        } else {
            *self.data.front().unwrap()
        }
    }

    fn get_rear(&self) -> i32 {
        if self.data.is_empty() {
            -1
        } else {
            *self.data.back().unwrap()
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn is_full(&self) -> bool {
        self.data.len() == self.maxsize
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
