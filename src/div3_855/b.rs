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
    let t: i32 = sc.sc();
    for _ in 0..t {
        let _: usize = sc.sc();
        let mut k: i32 = sc.sc();
        let s: String = sc.sc();
        let mut key = HashMap::new();
        let mut ans = 0;
        for c in s.chars() {
            let m = c.to_ascii_lowercase();
            let temp = key.entry(m).or_insert(0);
            if c.is_ascii_lowercase() {
                *temp += 1;
                if *temp <= 0 {
                    ans += 1;
                }
            } else {
                *temp -= 1;
                if *temp >= 0 {
                    ans += 1;
                }
            }
        }
        for (_, v) in key {
            if i32::abs(v) != 0 {
                let temp = i32::abs(v) / 2;
                ans += temp.min(k);
                k -= temp;
            }
            if k <= 0 {
                break;
            }
        }
        writeln!(out, "{}", ans).unwrap();
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
