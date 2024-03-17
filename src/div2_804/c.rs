#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
use std::cmp::*;
use std::collections::*;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

const c_mod: usize = 1e9 as usize + 7;

pub fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let t: usize = sc.sc();
    let mut ini = vec![0; 100005];
    let mut pos = vec![0; 100005];
    for _ in 0..t {
        let n: usize = sc.sc();
        for i in 0..n {
            ini[i] = sc.sc();
            pos[ini[i]] = i;
        }
        let mut l = pos[0];
        let mut r = pos[0];
        let mut res = 1;
        for i in 0..n {
            if pos[i] < l {
                l = pos[i];
            } else if pos[i] > r {
                r = pos[i];
            } else {
                res = res * (r - l + 1 - i) % c_mod;
            }
        }
        writeln!(out, "{}", res).unwrap();
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
