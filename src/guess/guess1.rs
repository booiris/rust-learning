use rand::Rng;
pub fn guess(_now: &Vec<char>) -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range('a' as u8, 'z' as u8 + 1) as char
}
