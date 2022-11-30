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

struct FreqStack {
    key: HashMap<i32, Vec<usize>>,
    index: usize,
    freq: HashMap<usize, Vec<i32>>,
    max_freq: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        FreqStack {
            key: HashMap::new(),
            index: 0,
            freq: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let now = self.key.entry(val).or_default();
        now.push(self.index);
        self.max_freq = self.max_freq.max(now.len());
        self.freq.entry(now.len()).or_default().push(val);
        self.index += 1;
    }

    fn pop(&mut self) -> i32 {
        if let Some(freq_index) = self.freq.get_mut(&self.max_freq) {
            let top = freq_index.pop().unwrap();
            while self.max_freq > 0 && self.freq.entry(self.max_freq).or_default().len() == 0 {
                self.max_freq -= 1;
            }
            if let Some(index) = self.key.get_mut(&top) {
                index.pop();
            }
            top
        } else {
            panic!()
        }
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

#[cfg(feature = "local")]
pub fn main() {
    let mut obj = FreqStack::new();
    obj.push(5);
    obj.push(7);
    obj.push(5);
    obj.push(7);
    obj.push(4);
    obj.push(5);

    println!("{}", obj.pop());
    println!("{}", obj.pop());
    println!("{}", obj.pop());
    println!("{}", obj.pop());
}
