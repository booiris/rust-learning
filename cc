#!/bin/bash

# build codeforce contest

dir=src/$1

if test -f $dir/mod.rs; then
    echo "$dir exist"
    exit 0
fi

mkdir $dir

touch $dir/mod.rs
echo "pub mod a;" >>$dir/mod.rs
echo "pub mod b;" >>$dir/mod.rs
echo "pub mod c;" >>$dir/mod.rs
echo "pub mod d;" >>$dir/mod.rs
echo "pub mod e;" >>$dir/mod.rs

main_context=$'#![allow(unused_imports)]
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
'
echo "$main_context" >$dir/a.rs
echo "$main_context" >$dir/b.rs
echo "$main_context" >$dir/c.rs
echo "$main_context" >$dir/d.rs
echo "$main_context" >$dir/e.rs

echo "pub mod $1;" >>src/lib.rs
