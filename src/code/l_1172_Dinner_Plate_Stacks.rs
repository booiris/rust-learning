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

struct DinnerPlates {
    data: Vec<Vec<i32>>,
    insert_index: usize,
    remove_index: usize,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            data: vec![vec![]; 200001],
            insert_index: 0,
            remove_index: 0,
            capacity: capacity as usize,
        }
    }

    fn push(&mut self, val: i32) {
        while self.data[self.insert_index].len() >= self.capacity {
            self.insert_index += 1;
        }
        self.data[self.insert_index].push(val);
        if self.data[self.insert_index].len() == self.capacity {
            self.insert_index += 1;
        }
        if self.remove_index < self.insert_index {
            self.remove_index = self.insert_index;
        }
    }

    fn pop(&mut self) -> i32 {
        while self.data[self.remove_index].is_empty() && self.remove_index != 0 {
            self.remove_index -= 1;
        }
        let val = self.data[self.remove_index].pop();
        if self.data[self.remove_index].is_empty() && self.remove_index != 0 {
            self.remove_index -= 1;
        }
        if self.remove_index < self.insert_index {
            self.insert_index = self.remove_index;
        }
        val.unwrap_or(-1)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        let val = self.data[index].pop();
        if val.is_some() && index < self.insert_index {
            self.insert_index = index;
            if self.remove_index < self.insert_index {
                self.remove_index = self.insert_index;
            }
        }
        val.unwrap_or(-1)
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
