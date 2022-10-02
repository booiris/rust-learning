pub trait Guess<'a> {
    fn guess(&mut self, now: &Vec<char>) -> char;
    fn end(&mut self);
    fn get_res(&mut self, _letter: char, _res: bool) {}
}
