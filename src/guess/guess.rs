pub trait Guess<'a> {
    fn guess(&self, now: &Vec<char>) -> char;
}
