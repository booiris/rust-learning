pub trait Guess<'a> {
    fn guess(&mut self, now: &Vec<char>) -> char;
    fn end(&mut self);
}
