// use std::cmp::*;
// use std::collections::*;
use std::io::BufRead;
use std::io::BufReader;
// use std::ops::Bound::*;
use guess_word::guess::guess::Guess;
use guess_word::guess::*;
use std::fs::File;
use std::{thread, time};

static DEBUG: bool = false;

fn main() {
    let data = get_data();
    let player = guess2::Player { data: &data };
    let (correct, wrong, sum) = simulate(&data, &player);
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

fn simulate<'a>(data: &Vec<String>, player: &impl Guess<'a>) -> (i32, i32, i32) {
    let (mut correct, mut wrong, mut sum) = (0, 0, 0);
    for word in data {
        sum += 1;

        if DEBUG {
            println!(
                "{} times Game! win!: {}  lose!: {}  word: {}",
                sum, correct, wrong, word
            );
        }

        match game(word, player) {
            true => correct += 1,
            false => wrong += 1,
        }
    }
    (correct, wrong, sum)
}

fn game<'a>(word: &String, guess: &impl Guess<'a>) -> bool {
    let char_list = word.chars().into_iter().collect::<Vec<_>>();
    let mut now = vec!['_'; char_list.len()];
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
            if x == c && now[i] == '_' {
                now[i] = x;
                guess_right = true;
            }
        }
        if !guess_right {
            wrong_cnt += 1;
        }
    }

    if wrong_cnt >= 6 {
        false
    } else {
        true
    }
}
