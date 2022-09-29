use counter::Counter;
use regex::Regex;

use super::guess::Guess;
pub struct Player<'a> {
    pub data: &'a Vec<String>,
    key: Vec<(char, usize)>,
    dict: Vec<String>,
    guessed_letter: Vec<char>,
}

impl<'a> Player<'a> {
    pub fn new(data: &'a Vec<String>) -> Self {
        let key = data
            .join("")
            .chars()
            .collect::<Counter<_>>()
            .most_common_ordered();

        Player {
            data,
            key,
            dict: data.clone(),
            guessed_letter: vec![],
        }
    }
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&mut self, now: &Vec<char>) -> char {
        let mut new_dict = Vec::<String>::new();
        let now = now.iter().collect::<String>();
        let re = Regex::new(&now).unwrap();
        let len = now.len();
        for word in &self.dict {
            if word.len() == len && re.is_match(word) {
                new_dict.push(word.clone());
            }
        }
        let key = new_dict
            .join("")
            .chars()
            .collect::<Counter<_>>()
            .most_common();
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
    }
}
