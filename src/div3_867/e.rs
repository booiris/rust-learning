#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n: usize = sc.sc();
    let ini: String = sc.sc();
    let ini = ini.chars().collect::<Vec<_>>();
    if n % 2 == 1 {
        writeln!(out, "-1");
        return;
    }
    let mut cnt = HashMap::new();
    let mut maxn = 0;
    for x in &ini {
        let temp = cnt.entry(*x).or_insert(0);
        *temp += 1;
        maxn = maxn.max(*temp);
    }
    if maxn > n / 2 {
        writeln!(out, "-1");
    }

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