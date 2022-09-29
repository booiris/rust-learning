use super::guess::Guess;
use rand::Rng;

pub struct Player<'a> {
    pub data: &'a Vec<String>,
}

impl<'a> Player<'a> {
    pub fn new(data: &'a Vec<String>) -> Self {
        Player { data }
    }
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&self, _now: &Vec<char>) -> char {
        let mut rng = rand::thread_rng();
        rng.gen_range('a' as u8, 'z' as u8 + 1) as char
    }
}
