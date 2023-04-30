#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n: usize = sc.sc();
    let mut res = vec![0; n];
    res[0] = n;
    let mut now = -1;
    let mut low = 1;
    let mut high = n - 1;
    let mut sum = 0;
    let mut key = HashSet::new();
    for x in &mut res {
        if *x == n {
            continue;
        }
        if now > 0 {
            if sum < low {
                *x = low - sum;
            } else if sum > low {
                *x = n + low - sum;
            } else {
                writeln!(out, "-1").unwrap();
                return;
            }
            low += 1;
        } else {
            if sum < high {
                *x = high - sum;
            } else if sum > high {
                *x = n + high - sum;
            } else {
                writeln!(out, "-1").unwrap();
                return;
            }
            high -= 1;
        }
        if key.contains(x) {
            writeln!(out, "-1").unwrap();
            return;
        }
        key.insert(*x);
        sum += *x;
        sum %= n;
        now = -now;
    }
    for x in res {
        write!(out, "{} ", x).unwrap();
    }
    writeln!(out).unwrap();
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
