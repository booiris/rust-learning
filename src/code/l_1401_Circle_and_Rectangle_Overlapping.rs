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

fn between(val: i32, l: i32, r: i32) -> bool {
    val >= l && val <= r
}

fn in_cycle(x: i32, y: i32, xc: i32, yc: i32, radius: i32) -> bool {
    (x - xc) * (x - xc) + (y - yc) * (y - yc) <= radius * radius
}

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        if between(x_center, x1 - radius, x2 + radius) && between(y_center, y1, y2) {
            return true;
        }
        if between(x_center, x1, x2) && between(y_center, y1 - radius, y2 + radius) {
            return true;
        }
        if between(x_center, x1 - radius, x1)
            && between(y_center, y2, y2 + radius)
            && in_cycle(x_center, y_center, x1, y2, radius)
        {
            return true;
        }
        if between(x_center, x1 - radius, x1)
            && between(y_center, y1 - radius, y1)
            && in_cycle(x_center, y_center, x1, y1, radius)
        {
            return true;
        }
        if between(x_center, x2, x2 + radius)
            && between(y_center, y2, y2 + radius)
            && in_cycle(x_center, y_center, x2, y2, radius)
        {
            return true;
        }
        if between(x_center, x2, x2 + radius)
            && between(y_center, y1 - radius, y1)
            && in_cycle(x_center, y_center, x2, y1, radius)
        {
            return true;
        }

        false
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
