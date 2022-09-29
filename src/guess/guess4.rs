use super::guess::Guess;
pub struct Player<'a> {
    pub data: &'a Vec<String>,
}

impl<'a> Player<'a> {
    pub fn new(data: &'a Vec<String>) -> Self {
        Player { data }
    }
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&mut self, _now: &Vec<char>) -> char {
        'a'
    }
    fn end(&mut self) {}
}
