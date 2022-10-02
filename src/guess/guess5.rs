use super::guess::Guess;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::*;
pub struct Player<'a> {
    pub data: &'a Vec<String>,
    key: Vec<(char, usize)>,
    dict: Vec<String>,
    guessed_letter: Vec<char>,
    res: (char, bool),
    chance: i32,
    bad_letters: Vec<char>,
    pre: Vec<(String, i32)>,
    back: Vec<(String, i32)>,
}

impl<'a> Player<'a> {
    pub fn new(data: &'a Vec<String>) -> Self {
        let mut letter_map = HashMap::new();
        for word in data {
            for letter in 'a'..='z' {
                if word.contains(letter) {
                    letter_map
                        .entry(letter)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }
        let mut key = letter_map.iter().map(|(x, y)| (*x, *y)).collect::<Vec<_>>();
        key.sort_by_key(|x| Reverse(x.1));
        Player {
            data,
            key,
            dict: data.clone(),
            guessed_letter: vec![],
            res: ('#', false),
            chance: 6,
            bad_letters: vec![],
            pre: Player::get_pre(data),
            back: Player::get_back(data),
        }
    }
    fn get_pre(data: &'a Vec<String>) -> Vec<(String, i32)> {
        let mut key = HashMap::<String, i32>::new();
        for word in data {
            for i in 2..word.len() {
                let sub = &word[0..i];
                key.entry(sub.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
        let mut pre = key
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| x.1 > 0)
            .collect::<Vec<_>>();
        pre.sort_by_key(|x| Reverse(x.1));
        pre
    }

    fn get_back(data: &'a Vec<String>) -> Vec<(String, i32)> {
        let mut key = HashMap::<String, i32>::new();
        for word in data {
            for i in 2..word.len() {
                let sub = &word[word.len() - i..word.len()];
                key.entry(sub.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
        let mut back = key
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| x.1 > 0)
            .collect::<Vec<_>>();
        back.sort_by_key(|x| Reverse(x.1));
        back
    }

    fn get_key_letters(&self, data: &'a Vec<String>) -> Vec<(char, f64)> {
        let mut letter_map = HashMap::new();
        for word in data {
            for letter in ['a', 'e', 'i', 'o', 'u', 'y'] {
                if word.contains(letter) {
                    letter_map
                        .entry(letter)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }
        let letters = letter_map.iter().map(|(x, y)| (*x, *y)).collect::<Vec<_>>();

        if letters.len() == 1 {
            return vec![((letters[0].0), 0.0)];
        }

        let len = data.len() as f64;
        let mut letters = letters
            .iter()
            .map(|(x, y)| {
                if *y == data.len() {
                    return (*x, 10000.0);
                }
                let temp = *y as f64 / len;
                (*x, -temp * temp.log2() - (1.0 - temp) * (1.0 - temp).log2())
            })
            .collect::<Vec<_>>();
        letters.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
        letters
    }

    fn use_key_letter(&self, res: &mut char) {
        let key_letters = self.get_key_letters(&self.dict);

        for x in key_letters {
            if self.guessed_letter.iter().find(|&&y| y == x.0).is_none() {
                *res = x.0;
                break;
            }
        }
    }

    fn use_pre_back(&self, res: &mut char, now: &Vec<char>) {
        let mut key = HashMap::<char, i32>::new();

        for x in &self.pre {
            if x.0.len() > now.len() {
                continue;
            }
            let re = Regex::new(&now[0..x.0.len()].iter().collect::<String>()).unwrap();
            if x.0.contains(&self.bad_letters[..]) || !re.is_match(&x.0) {
                continue;
            }
            for w in x.0.chars() {
                key.entry(w).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        for x in &self.back {
            if x.0.len() > now.len() {
                continue;
            }
            let re = Regex::new(
                &now[now.len() - x.0.len()..now.len()]
                    .iter()
                    .collect::<String>(),
            )
            .unwrap();
            if x.0.contains(&self.bad_letters[..]) || !re.is_match(&x.0) {
                continue;
            }
            for w in x.0.chars() {
                key.entry(w).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let mut key = key.into_iter().collect::<Vec<_>>();
        key.sort_by_key(|x| Reverse(x.1));
        for (letter, _) in &key {
            if self.guessed_letter.iter().find(|&x| x == letter).is_none() {
                *res = *letter;
                break;
            }
        }
    }

    fn use_freq(&self, res: &mut char) {
        let mut letter_map = HashMap::new();
        for word in &self.dict {
            for letter in 'a'..='z' {
                if word.contains(letter) {
                    letter_map
                        .entry(letter)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }
        let mut key = letter_map.iter().map(|(x, y)| (*x, *y)).collect::<Vec<_>>();
        key.sort_by_key(|x| Reverse(x.1));
        for (letter, _) in &key {
            if self.guessed_letter.iter().find(|&x| x == letter).is_none() {
                *res = *letter;
                break;
            }
        }
    }

    fn use_final(&self, res: &mut char) {
        for (letter, _) in &self.key {
            if self.guessed_letter.iter().find(|&x| x == letter).is_none() {
                *res = *letter;
                break;
            }
        }
    }
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&mut self, now: &Vec<char>) -> char {
        let mut res = '!';

        // println!(
        //     "{:?} {} {}",
        //     self.res,
        //     self.chance,
        //     now.iter().collect::<String>()
        // );

        let mut new_dict = Vec::<String>::new();
        for word in &self.dict {
            if self.res.1 && word.contains(self.res.0) {
                new_dict.push(word.clone());
            }
            if !self.res.1 && !word.contains(self.res.0) {
                new_dict.push(word.clone());
            }
        }
        self.dict = new_dict;

        // if self.chance > 2 {
        //     self.use_key_letter(&mut res);
        // }

        if res == '!' {
            self.use_pre_back(&mut res, now);
        }

        if res == '!' {
            self.use_freq(&mut res);
        }

        if res == '!' {
            self.use_final(&mut res);
        }

        self.guessed_letter.push(res);
        res
    }

    fn end(&mut self) {
        self.guessed_letter.clear();
        self.dict = self.data.clone();
        self.res = ('#', false);
        self.chance = 6;
        self.bad_letters = vec![];
    }

    fn get_res(&mut self, letter: char, res: bool) {
        self.res = (letter, res);
        if !res {
            self.chance -= 1;
            self.bad_letters.push(letter);
        }
    }
}
