use std::cmp::Reverse;
// use std::cmp::*;
use std::collections::*;
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

pub static DEBUG: bool = false;

static mut PRIOD: i32 = 10;

fn _main() {
    let train_data = get_data("../data/data.txt");

    let mut letter_map = HashMap::new();
    for word in &train_data {
        for letter in 'a'..='z' {
            if word.contains(letter) {
                letter_map
                    .entry(letter)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
    let mut letters = letter_map.iter().map(|(x, y)| (*x, *y)).collect::<Vec<_>>();
    letters.sort_by_key(|x| Reverse(x.1));
    println!("{:?}", letters);
    let len = train_data.len() as f64;
    let mut letters = letters
        .iter()
        .map(|(x, y)| {
            let temp = *y as f64 / len;
            (x, -temp * temp.log2() - (1.0 - temp) * (1.0 - temp).log2())
        })
        .collect::<Vec<_>>();
    letters.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
    println!("{:?}", letters);

    let p_len = 4;
    let mut pre = HashMap::<String, i32>::new();
    for x in &train_data {
        if x.len() < p_len {
            continue;
        }
        pre.entry(x[0..p_len].to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut end = HashMap::<String, i32>::new();
    for x in &train_data {
        let len = x.len();
        if len < p_len {
            continue;
        }
        end.entry(x[len - p_len..len].to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let pre = pre.iter().collect::<Vec<_>>();
    let mut pre = pre.iter().filter(|x| *x.1 > 5).collect::<Vec<_>>();
    let end = end.iter().collect::<Vec<_>>();
    let mut end = end.iter().filter(|x| *x.1 > 5).collect::<Vec<_>>();

    println!("{} {}", pre.len(), end.len());
    pre.sort_by_key(|x| Reverse(x.1));
    end.sort_by_key(|x| Reverse(x.1));

    println!("{:?}", &pre[0..pre.len().min(30)]);
    println!("{:?}", &pre[0.max(pre.len() - 30)..pre.len()]);
    println!("{:?}", &end[0..end.len().min(30)]);
    println!("{:?}", &pre[0.max(end.len() - 30)..end.len()]);
}

fn main() {
    let train_data = get_data("../data/data.txt");
    let test_data = get_data("../data/test.txt");
    let s1 = HashSet::<_>::from_iter(train_data.into_iter().clone());
    let s2 = HashSet::<_>::from_iter(test_data.into_iter().clone());
    let s = s1
        .intersection(&s2)
        .map(|x| x.clone())
        .collect::<HashSet<_>>();
    let key = s2.difference(&s).map(|x| x.clone()).collect::<Vec<_>>();
    println!(
        "{} {} {} {} {}",
        s.len(),
        key.len(),
        s1.len(),
        s2.len(),
        s.len() as f64 / s2.len() as f64 * 100.0
    );

    let mut cpu_num = num_cpus::get();

    if DEBUG {
        cpu_num = 1;
    }

    let train_data = Arc::new(get_data("../data/data.txt"));
    let test_data = Arc::new(key);

    let len = test_data.len() / cpu_num;
    let (mut s, mut e) = (0 as usize, len);
    let (tx, rx) = mpsc::channel();

    unsafe {
        if DEBUG {
            PRIOD = 1;
        }
    }

    for i in 0..cpu_num {
        if i == cpu_num - 1 {
            e = test_data.len();
        }

        let share_data = Arc::clone(&train_data);
        let sub_data = test_data[s..e].to_vec();
        let thread_tx = tx.clone();

        thread::spawn(move || {
            let mut player = guess5::Player::new(&share_data);
            let mut cnt = 0;
            let (correct, wrong, sum) = simulate(&sub_data, &mut player, i, &mut cnt);

            thread_tx.send((correct, wrong, sum)).unwrap();
        });

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

fn get_data(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("no such file");
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
            if DEBUG {
                println!("word: {}", word);
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
            // println!(
            //     "now guess: {}, now get: {}",
            //     c,
            //     now.iter().collect::<String>()
            // );
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
        guess.get_res(c, guess_right);
    }

    guess.end();

    if wrong_cnt >= 6 {
        false
    } else {
        true
    }
}
