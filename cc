#!/bin/bash

# build leetcode contest

dir=src/contest/$1

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

main_context=$'#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[cfg(test)]
pub fn main() {
    println!("res:");
}

'
echo "$main_context" >$dir/a.rs
echo "$main_context" >$dir/b.rs
echo "$main_context" >$dir/c.rs
echo "$main_context" >$dir/d.rs

echo "pub mod $1;" >>src/contest/mod.rs
