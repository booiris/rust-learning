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
fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n = sc.sc();
    let ini = (0..n).map(|_| sc.sc()).collect::<Vec<i32>>();
    let mut res = i32::MIN;
    let mut l = 0;
    let mut r = 0;
    let mut now = 0;
    let mut key = 0;
    while r < n {
        while (r == n - 1) || (r < n && ini[r].abs() % 2 != ini[r + 1].abs() % 2) {
            now += ini[r];
            if res < now {
                key = r;
                res = now;
            }
            if now < 0 {
                break;
            }
            r += 1;
        }
        if now < 0 {
            r += 1;
        } else if r < n {
            now += ini[r];
            res = res.max(now);
            r += 1;
        }
        now = res;
        while l < r && l < n {
            if l <= key {
                now -= ini[l];
            }
            if l != r - 1 {
                res = res.max(now);
            }
            l += 1;
        }
        now = 0;
    }
    writeln!(out, "{res}");
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
