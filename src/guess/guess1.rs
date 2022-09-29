#![allow(dead_code)]
use super::guess::Guess;
use rand::Rng;

pub struct Player<'a> {
    pub data: &'a Vec<String>,
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&self, _now: &Vec<char>) -> char {
        let mut rng = rand::thread_rng();
        rng.gen_range('a' as u8, 'z' as u8 + 1) as char
    }
}
