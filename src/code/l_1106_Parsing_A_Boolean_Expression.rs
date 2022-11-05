#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
#[cfg(feature = "local")]
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut expr = vec![];
        let mut op: Vec<char> = vec![];

        for x in expression.chars() {
            match x {
                't' => expr.push(true),
                'f' => expr.push(false),
                '!' | '&' | '|' | '(' | ',' => op.push(x),
                ')' => {
                    let mut temp = vec![];
                    while let Some(top) = op.pop() {
                        temp.push(expr.pop().unwrap());
                        if top == '(' {
                            break;
                        }
                    }
                    // println!("{:?}", temp);
                    let opc = op.pop().unwrap();
                    match opc {
                        '!' => expr.push(!temp[0]),
                        '&' => expr.push(temp.into_iter().fold(true, |now, x| now && x)),
                        '|' => expr.push(temp.into_iter().fold(false, |now, x| now || x)),
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }

        expr[0]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let s = "|(&(t,f,t),!(t))".to_owned();
    println!("res:{}", Solution::parse_bool_expr(s));
}
