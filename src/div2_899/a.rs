#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

fn _gcd<T: Default + std::marker::Copy + std::ops::Rem<Output = T> + std::cmp::PartialEq>(
    a: T,
    b: T,
) -> T {
    if b == T::default() {
        return a;
    }
    _gcd(b, a % b)
}

fn ok(mid: i64, a: &[i64]) -> bool {
    let mut now = 0;
    for x in a {
        now += 1;
        while now == *x && now <= mid {
            now += 1;
        }
        if now > mid {
            return false;
        }
    }
    true
}

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n: usize = sc.sc();
    let a = (0..n).map(|_| sc.sc()).collect::<Vec<i64>>();
    let mut l = 1;
    let mut r = 1e9 as i64 + 5;
    while l <= r {
        let mid = (l + r) / 2;
        if ok(mid, &a) {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    writeln!(out, "{l}");
}

pub fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let t: i32 = sc.sc();
    for _ in 0..t {
        solve(&mut sc, &mut out);
    }
}
struct Scanner<B> {
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
