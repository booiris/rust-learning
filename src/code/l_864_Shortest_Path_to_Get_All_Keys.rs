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

const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let mut res = i32::MAX;
        let grid = grid
            .into_iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (n, m) = (grid.len(), grid[0].len());
        let mut key = HashMap::<(usize, usize, i32), i32>::new();
        let mut q = VecDeque::new();
        let mut key_id = HashMap::new();
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '@' {
                    q.push_back((i, j, 0, 0));
                    key.insert((i, j, 0), 0);
                }
                if grid[i][j].is_ascii_lowercase() {
                    if key_id.get(&grid[i][j]).is_none() {
                        key_id.insert(grid[i][j], cnt);
                        cnt += 1;
                    }
                }
            }
        }
        let aim = (1 << key_id.len()) - 1;
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let now = q.pop_front().unwrap();
                // println!("{:?}", now);
                let bring_key = now.2;
                if bring_key == aim {
                    res = res.min(now.3);
                    continue;
                }
                if now.3 >= res {
                    continue;
                }
                for dir in DIR {
                    let (nx, ny) = (now.0 as i32 + dir.0, now.1 as i32 + dir.1);
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    let mut n_key = bring_key;
                    if grid[nx][ny] == '#' {
                        continue;
                    }
                    if grid[nx][ny].is_ascii_uppercase() {
                        let need_key = grid[nx][ny] as u8 | 0x20;
                        let need_key = key_id.get(&(need_key as char)).unwrap();
                        if bring_key & (1 << need_key) == 0 {
                            continue;
                        }
                    }
                    if grid[nx][ny].is_ascii_lowercase() {
                        let id = key_id.get(&grid[nx][ny]).unwrap();
                        n_key |= 1 << id;
                    }
                    if let Some(step) = key.get(&(nx, ny, n_key)) {
                        if step <= &(now.3 + 1) {
                            continue;
                        }
                    }
                    key.insert((nx, ny, n_key), now.3 + 1);
                    q.push_back((nx, ny, n_key, now.3 + 1));
                }
            }
        }
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![
        "#..#.#.#..#.#.#.....#......#..",
        ".#.......#....#A.....#.#......",
        "#....#.....#.........#........",
        "...#.#.........#..@....#....#.",
        ".#.#.##...#.........##....#..#",
        "..........#..#..###....##..#.#",
        ".......#......#...#...#.....c#",
        ".#...#.##......#...#.###...#..",
        "..........##...#.......#......",
        "#...#.........a#....#.#.##....",
        "..#..#...#...#..#....#.....##.",
        "..........#...#.##............",
        "...#....#..#.........#..D.....",
        "....#E.#....##................",
        "...........##.#.......#.#....#",
        "...#..#...#.#............#e...",
        "..#####....#.#...........##..#",
        "##......##......#.#...#..#.#..",
        ".#F.......#..##.......#....#..",
        "............#....#..#..#...#..",
        ".............#...#f...#..##...",
        "....#..#...##.........#..#..#.",
        ".....#.....##.###..##.#......#",
        ".#..#.#...#.....#........###..",
        ".....#.#...#...#.....#.....#..",
        "##.....#....B.....#..#b.......",
        ".####....##..#.##..d.#......#.",
        "..#.....#....##........##...##",
        "...#...#...C..#..#....#.......",
        "#.....##.....#.#......#.......",
    ];
    let a = a.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    println!("res:{}", Solution::shortest_path_all_keys(a));
}
