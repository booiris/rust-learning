use std::io::stdin;
static mut INPUT: String = String::new();
macro_rules! cin {
    ($type:ty) => {{
        stdin().read_line(&mut INPUT).expect("failed to read_line");
        let x = INPUT.trim().parse::<$type>().unwrap();
        x
    }};
}
pub fn main() {
    let mut input = String::new();
    let mut x = cin!(i32);
}
