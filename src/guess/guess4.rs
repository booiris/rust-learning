use counter::Counter;
use regex::Regex;
use std::collections::*;
use std::time::Instant;

use super::guess::Guess;
pub struct Player<'a> {
    pub data: &'a Vec<String>,
    key: Vec<(char, usize)>,
    dict: Vec<String>,
    guessed_letter: Vec<char>,
    _key_dict: HashMap<String, i32>,
    _vowel: Vec<char>,
    times: i32,
}

impl<'a> Player<'a> {
    pub fn new(data: &'a Vec<String>) -> Self {
        let key = data
            .join("")
            .chars()
            .collect::<Counter<_>>()
            .most_common_ordered();

        let mut key_dict = HashMap::<String, i32>::new();
        let vowel = vec!['a', 'e', 'i', 'o', 'u', 'y'];

        let start = Instant::now();

        for i in vowel.iter() {
            for j in vowel.iter() {
                for k in vowel.iter() {
                    let temp = vec![i.clone(), j.clone(), k.clone()];
                    let s = temp.iter().collect::<String>();
                    for word in data {
                        if word.contains(&temp[..]) {
                            key_dict
                                .entry(s.clone())
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }
                    }
                }
            }
        }

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        println!("{:?}", key_dict.get(&"abc".to_string()));

        Player {
            data,
            key,
            dict: data.clone(),
            guessed_letter: vec![],
            _key_dict: key_dict,
            _vowel: vowel,
            times: 0,
        }
    }
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&mut self, now: &Vec<char>) -> char {
        let mut new_dict = Vec::<String>::new();
        let now = now.iter().collect::<String>();
        let re = Regex::new(&now).unwrap();
        let len = now.len();
        let mut cnt = HashMap::<char, i32>::new();
        for word in &self.dict {
            if word.len() == len && re.is_match(word) {
                let temp = word.chars().collect::<HashSet<_>>();
                for c in temp {
                    cnt.entry(c).and_modify(|c| *c += 1).or_insert(1);
                }
                new_dict.push(word.clone());
            }
        }

        let mut key = vec![];
        let len = new_dict.len();
        if len != 0 {
            for x in cnt {
                if x.1 == 0 {
                    continue;
                }
                if x.1 == len as i32 {
                    key.push((x.0, 100000000.0));
                    continue;
                }
                let aim = x.1 as f64 / len as f64;
                let aim = -aim * aim.log2() - (1.0 - aim) * (1.0 - aim).log2();
                key.push((x.0, aim));
            }
            key.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        }

        let mut res = '!';
        for (letter, _) in key {
            if self.guessed_letter.iter().find(|&&x| x == letter).is_none() {
                res = letter;
                break;
            }
        }

        if res == '!' {
            for (letter, _) in &self.key {
                if self.guessed_letter.iter().find(|&x| x == letter).is_none() {
                    res = *letter;
                    break;
                }
            }
        }

        self.dict = new_dict;
        self.guessed_letter.push(res);
        res
    }

    fn end(&mut self) {
        self.guessed_letter.clear();
        self.dict = self.data.clone();
        self.times = 0;
    }
}
