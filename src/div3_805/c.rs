#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

pub fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let t = sc.sc();
    let mut key1 = HashMap::<i32, usize>::new();
    let mut key2 = HashMap::<i32, usize>::new();
    for _ in 0..t {
        let n = sc.sc();
        let m = sc.sc();
        let ini: Vec<i32> = (0..n).map(|_| sc.sc()).collect();
        let q: Vec<(i32, i32)> = (0..m).map(|_| (sc.sc(), sc.sc())).collect();
        key1.clear();
        key2.clear();
        for (i, x) in ini.iter().enumerate() {
            if let Some(index) = key1.get(x) {
                if i < *index {
                    key1.insert(*x, i);
                }
            } else {
                key1.insert(*x, i);
            }
            if let Some(index) = key2.get(x) {
                if i > *index {
                    key2.insert(*x, i);
                }
            } else {
                key2.insert(*x, i);
            }
        }
        for x in q.iter() {
            if let (Some(l), Some(r)) = (key1.get(&x.0), key2.get(&x.1)) {
                if l <= r {
                    writeln!(out, "YES").unwrap();
                } else {
                    writeln!(out, "NO").unwrap();
                }
            } else {
                writeln!(out, "NO").unwrap();
            }
        }
    }
}
pub struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}
impl<B: BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }
    pub fn sc<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
