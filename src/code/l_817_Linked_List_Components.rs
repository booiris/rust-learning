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

// Definition for singly-linked list.
#[cfg(feature = "local")]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[cfg(feature = "local")]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn num_components(mut head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let key = nums.iter().collect::<HashSet<_>>();
        let mut res = 0;
        let mut flag = false;
        while let Some(mut x) = head {
            if key.get(&x.val).is_some() {
                flag = true;
            } else {
                if flag {
                    res += 1;
                }
                flag = false;
            }
            head = x.next.take();
        }
        if flag {
            res += 1;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
