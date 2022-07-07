#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
pub fn main() {
    let mut scan = Scanner::default();
    let t = scan.next::<i32>();
}
