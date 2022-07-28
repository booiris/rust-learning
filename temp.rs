use std::mem;
fn main() {
    println!("{}", mem::size_of::<usize>());
}

// let ini: Vec<i32> = (0..n).map(|_| sc.sc()).collect();
// let q: Vec<(i32, i32)> = (0..m).map(|_| (sc.sc(), sc.sc())).collect();
// ini.iter().for_each(|x| write!(out, "{} ", x).unwrap());
// write!(out, "\n").unwrap();
// q.iter().for_each(|x| writeln!(out, "{:?}", x).unwrap());