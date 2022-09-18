#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::mem::swap;
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

// Definition for a binary tree node.
#[cfg(feature = "local")]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[cfg(feature = "local")]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn reverse_odd_levels(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q = VecDeque::new();
        q.push_back(root.as_ref().unwrap().clone());
        let mut d = 0;
        while !q.is_empty() {
            let len = q.len();
            let mut key = VecDeque::new();
            for _ in 0..len {
                let now = q.pop_front().unwrap();
                if now.borrow().left.is_some() {
                    q.push_back(now.borrow().left.clone().unwrap());
                }
                if now.borrow().right.is_some() {
                    q.push_back(now.borrow().right.clone().unwrap());
                }
                if d % 2 == 1 {
                    key.push_back(now);
                }
            }
            if d % 2 == 1 {
                while !key.is_empty() {
                    let f = key.pop_back();
                    let b = key.pop_front();
                    if let (Some(k1), Some(k2)) = (f, b) {
                        swap(&mut k1.borrow_mut().val, &mut k2.borrow_mut().val);
                    }
                }
            }
            d += 1;
        }
        root.take()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
