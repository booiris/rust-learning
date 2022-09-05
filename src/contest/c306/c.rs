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

fn dfs(p: &String, vis: &mut [bool; 9], res: &mut String, depth: usize) -> bool {
    if depth == p.len() {
        return true;
    }
    let (s, e): (char, char);
    if p.chars().nth(depth).unwrap() == 'I' {
        s = ((*res).chars().nth(depth).unwrap() as u8 + 1) as char;
        e = '9';
    } else {
        s = '1';
        e = ((*res).chars().nth(depth).unwrap() as u8 - 1) as char;
    }
    for x in s..=e {
        if vis[x as usize - '1' as usize] {
            continue;
        }
        (*res).push(x);
        vis[x as usize - '1' as usize] = true;
        if dfs(p, vis, res, depth + 1) {
            return true;
        }
        vis[x as usize - '1' as usize] = false;
        (*res).pop();
    }
    false
}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut vis = [false; 9];
        for x in '1'..='9' {
            vis[x as usize - '1' as usize] = true;
            let mut res = x.to_string();
            if dfs(&pattern, &mut vis, &mut res, 0) {
                return res;
            }
            vis[x as usize - '1' as usize] = false;
        }
        "".to_string()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res: {}", Solution::smallest_number("IIIDIDDD".to_string()));
}
