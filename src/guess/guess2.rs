use super::guess::Guess;
pub struct Player<'a> {
    pub data: &'a Vec<String>,
}

impl<'a> Guess<'a> for Player<'a> {
    fn guess(&self, _now: &Vec<char>) -> char {
        'a'
    }
}
