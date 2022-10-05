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

// impl Solution {
//     pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
//         let mut res = HashMap::<String, i32>::new();
//         for x in cpdomains {
//             let mut x = x.split_ascii_whitespace();
//             let num = x.next().unwrap().parse::<i32>().unwrap();
//             let x = x.next().unwrap();
//             let mut x = x.split('.').collect::<Vec<_>>();
//             x.reverse();
//             let mut now = "".to_string();
//             for y in x {
//                 if now != "" {
//                     now = y.to_string() + "." + &now;
//                 } else {
//                     now = y.to_string();
//                 }

//                 res.entry(now.clone())
//                     .and_modify(|c| *c += num)
//                     .or_insert(num);
//             }
//         }
//         res.into_iter()
//             .map(|(x, y)| y.to_string() + " " + &x)
//             .collect::<Vec<_>>()
//     }
// }

use std::collections::HashMap;
impl Solution {
    
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut h = HashMap::new();
        for s in cpdomains {
            let a: Vec<_> = s.split_whitespace().collect();
            let c = a[0].parse::<i32>().unwrap();
            let a = a[1];
            let mut i = 0;
            while i < a.len() {
                *h.entry(a[i..].to_owned()).or_insert(0) += c;
                i = i + a[i..].find('.').unwrap_or(a.len()) + 1;
            }
        }
        h.iter().map(|(k, v)| v.to_string() + " " + k).collect()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![
        "900 google.mail.com".to_string(),
        "50 yahoo.com".to_string(),
        "1 intel.mail.com".to_string(),
        "5 wiki.org".to_string(),
    ];
    println!("res:{:?}", Solution::subdomain_visits(a));
}
