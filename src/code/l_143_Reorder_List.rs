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
    // pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    //     let mut key = vec![];
    //     let mut now = head.take();
    //     while let Some(mut x) = now {
    //         now = x.next.take();
    //         key.push(Some(x));
    //     }

    //     let len = key.len();
    //     let mid = key.len() / 2;
    //     let mut dummy = Box::new(ListNode::new(0));
    //     let mut now = &mut dummy;
    //     for i in 0..mid {
    //         now.next = key[i].take();
    //         now = now.next.as_mut().unwrap();
    //         now.next = key[len - i - 1].take();
    //         now = now.next.as_mut().unwrap();
    //     }

    //     if mid < len - mid {
    //         now.next = key[mid].take();
    //     }

    //     *head = dummy.next;

    // }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut real = head.take();
        let mut fast = real.clone();
        let mut slow = &mut real;

        while let Some(x) = fast {
            if x.next.is_none() {
                break;
            }
            slow = &mut slow.as_mut().unwrap().next;
            fast = x.next.unwrap().next;
        }

        slow = &mut slow.as_mut().unwrap().next;

        let mut slow = slow.take();
        let mut pre = None;
        while let Some(mut x) = slow.take() {
            slow = x.next;
            x.next = pre;
            pre = Some(x);
        }

        let mut front = real.as_mut().unwrap();
        let mut end = pre;
        while let Some(mut x) = end {
            end = x.next.take();
            x.next = front.next.take();
            front.next = Some(x);
            match front.next.as_mut().unwrap().next.as_mut() {
                Some(nxt) => front = nxt,
                None => break,
            }
        }

        *head = real;
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
