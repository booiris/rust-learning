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

main_context=$'pub fn main(){\n\n}\n'
echo "$main_context" >$dir/a.rs
echo "$main_context" >$dir/b.rs
echo "$main_context" >$dir/c.rs
echo "$main_context" >$dir/d.rs
echo "$main_context" >$dir/e.rs

echo "mod $1;" >> src/lib.rs
