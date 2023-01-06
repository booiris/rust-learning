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

struct ExamRoom {
    s: HashMap<i32, (i32, i32)>,
    key: BinaryHeap<(i32, Reverse<i32>, i32)>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    fn new(n: i32) -> Self {
        ExamRoom {
            s: HashMap::new(),
            key: BinaryHeap::new(),
            n: n - 1,
        }
    }

    fn seat(&mut self) -> i32 {
        println!("{:?}", self.key);
        println!("{:?}", self.s);
        while let Some((key, l, r)) = self.key.pop() {
            let l = l.0;
            if self.s.get(&l).is_none() || !self.s.get(&r).is_none() {
                continue;
            }
            let now = l + key;
            self.s.insert(now, (l, r));
            self.key.push(((now - l) / 2, Reverse(l), now));
            self.key.push(((r - now) / 2, Reverse(now), r));
            return now;
        }
        if self.s.len() == 0 {
            self.s.insert(0, (0, 0));
            return 0;
        }
        self.s.insert(self.n, (0, 0));
        self.key.push((self.n / 2, Reverse(0), self.n));
        self.n
    }

    fn leave(&mut self, p: i32) {
        self.s.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

#[cfg(feature = "local")]
pub fn main() {
    let mut a = ExamRoom::new(10);
    println!("res:{}", a.seat());
    println!("res:{}", a.seat());
    println!("res:{}", a.seat());
    println!("res:{}", a.seat());
    a.leave(4);
    println!("res:{}", a.seat());
}
