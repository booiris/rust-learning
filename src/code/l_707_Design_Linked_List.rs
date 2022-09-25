#![allow(dead_code, unused_imports, unused_macros)]
use std::cell::Ref;
use std::cell::RefCell;
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
use std::rc::Rc;
#[cfg(feature = "local")]
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

struct MyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: i32,
}

struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.len || index < 0 {
            return -1;
        }
        let mut now = &self.head;
        let mut x: &Option<Rc<RefCell<Node>>>;
        for _ in 0..index {
            x = &now.as_ref().unwrap().clone().borrow().next.clone();
            now = x;
        }
        0
        // now.as_ref().unwrap().borrow().val
    }

    fn add_at_head(&mut self, val: i32) {
        let mut now = Box::new(Node {
            val,
            next: self.head.take(),
        });
        // self.len += 1;
        // self.head = Some(now);
        // if self.tail.is_none() {
        //     self.tail = Some(now);
        // }
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut now = Box::new(Node { val, next: None });
        self.len += 1;
    }

    fn add_at_index(&self, index: i32, val: i32) {}

    fn delete_at_index(&self, index: i32) {}
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
