use std::borrow::Borrow;
#[allow(dead_code)]
#[allow(unused_imports)]
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
struct OrderedStream {
    head: Option<Box<Node>>,
    index: i32,
}

struct Node {
    nxt: Option<Box<Node>>,
    id: i32,
    val: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            head: Some(Box::new(Node {
                nxt: None,
                id: 0,
                val: "".to_string(),
            })),
            index: 1,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let now = self.head.take();
        let pre = &now;
        loop {
            match now {
                Some(x) => {
                    if x.id > id_key {
                        break;
                    }
                }
                None => break,
            }
            pre = &now;
        }
        let new_node = Box::new(Node {
            nxt: now,
            id: id_key,
            val: value,
        });

        pre.unwrap().nxt = Some(new_node);

        let res = vec![];
        now = self.head.unwrap().nxt;
        loop {
            match now {
                Some(x) => {
                    if x.id == self.index {
                        res.push(x.val);
                        self.index += 1;
                    }
                    now = x.nxt;
                }
                None => break,
            }
        }

        res
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

#[cfg(feature = "local")]
pub fn main() {
    let obj = OrderedStream::new(5);
    let ret_1: Vec<String> = obj.insert(3, "c".to_string());
    let ret_2: Vec<String> = obj.insert(1, "a".to_string());
    let ret_3: Vec<String> = obj.insert(2, "b".to_string());
    let ret_4: Vec<String> = obj.insert(5, "e".to_string());
    let ret_5: Vec<String> = obj.insert(4, "d".to_string());
    println!("{:?}", ret_1);
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
    println!("{:?}", ret_4);
    println!("{:?}", ret_5);
}
