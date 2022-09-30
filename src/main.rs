// use std::cmp::*;
// use std::collections::*;
extern crate num_cpus;
use std::io::BufRead;
use std::io::BufReader;
// use std::ops::Bound::*;
use guess_word::guess::guess::Guess;
use guess_word::guess::*;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use std::{thread, time};

static DEBUG: bool = false;

static mut PRIOD: i32 = 10;
fn main() {
    let cpu_num = num_cpus::get() - 1;
    let data = Arc::new(get_data());
    let len = data.len() / cpu_num;
    let (mut s, mut e) = (0 as usize, len);
    let (tx, rx) = mpsc::channel();

    unsafe {
        if DEBUG {
            PRIOD = 1;
        }
    }

    for i in 0..cpu_num {
        if i == cpu_num - 1 {
            e = data.len();
        }

        let share_data = Arc::clone(&data);
        let sub_data = data[s..e].to_vec();
        let thread_tx = tx.clone();

        thread::spawn(move || {
            let mut player = guess2::Player::new(&share_data);
            let mut cnt = 0;
            let (correct, wrong, sum) = simulate(&sub_data, &mut player, i, &mut cnt);

            thread_tx.send((correct, wrong, sum)).unwrap();
        });

        // handles.push(handle);
        (s, e) = (s + len, e + len);
    }

    let (mut correct, mut wrong, mut sum) = (0, 0, 0);
    for _ in 0..cpu_num {
        let res = rx.recv().unwrap();
        correct += res.0;
        wrong += res.1;
        sum += res.2;
    }
    println!("{} {} {}", correct, wrong, sum);
    println!("final: {}%", correct as f64 / sum as f64 * 100.0);
}

fn get_data() -> Vec<String> {
    let file = File::open("../data/data.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<_>>()
}

fn simulate<'a>(
    data: &Vec<String>,
    player: &mut impl Guess<'a>,
    cpu_index: usize,
    cnt: &mut i32,
) -> (i32, i32, i32) {
    let (mut correct, mut wrong, mut sum) = (0, 0, 0);
    for word in data {
        sum += 1;

        unsafe {
            if *cnt == PRIOD {
                println!(
                    "cpu_index: {}; {} times Game! win!: {}  lose!: {} ",
                    cpu_index, sum, correct, wrong
                );
                *cnt = 0;
            }
            *cnt += 1;
        }

        match game(word, player) {
            true => correct += 1,
            false => wrong += 1,
        }
    }
    (correct, wrong, sum)
}

fn game<'a>(word: &String, guess: &mut impl Guess<'a>) -> bool {
    let char_list = word.chars().into_iter().collect::<Vec<_>>();
    let mut now = vec!['.'; char_list.len()];
    let mut wrong_cnt = 0;

    while wrong_cnt < 6 && now != char_list {
        let c = guess.guess(&now);

        if DEBUG {
            println!(
                "now guess: {}, now get: {}",
                c,
                now.iter().collect::<String>()
            );
            thread::sleep(time::Duration::from_secs_f32(0.5));
        }

        let mut guess_right = false;
        for (i, x) in word.chars().enumerate() {
            if x == c && now[i] == '.' {
                now[i] = x;
                guess_right = true;
            }
        }
        if !guess_right {
            wrong_cnt += 1;
        }
    }

    guess.end();

    if wrong_cnt >= 6 {
        false
    } else {
        true
    }
}
