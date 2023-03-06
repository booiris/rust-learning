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
        let n: usize = sc.sc();
        let s: String = sc.sc();
        let s = s.to_ascii_lowercase().chars().collect::<Vec<_>>();
        let mut flag = true;
        let mut cnt = 0;
        let mut index = 0;
        while index < n && s[index] == 'm' {
            index += 1;
            cnt += 1;
        }
        if cnt == 0 {
            flag = false;
        }
        cnt = 0;
        while index < n && s[index] == 'e' {
            index += 1;
            cnt += 1;
        }
        if cnt == 0 {
            flag = false;
        }
        cnt = 0;
        while index < n && s[index] == 'o' {
            index += 1;
            cnt += 1;
        }
        if cnt == 0 {
            flag = false;
        }
        cnt = 0;
        while index < n && s[index] == 'w' {
            index += 1;
            cnt += 1;
        }
        if cnt == 0 {
            flag = false;
        }
        if index != n{
            flag = false;
        }
        if flag {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
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
