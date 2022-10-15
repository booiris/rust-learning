#![allow(unused_macros)]
use super::q_learning::*;
use super::utils;
use std::ops::Sub;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MOVE: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
#[derive(Copy, Clone, Debug)]
pub enum Move {
    Right = 0,
    Left = 1,
    Down = 2,
    Up = 3,
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(v: (u32, u32)) -> Point {
        Point { x: v.0, y: v.1 }
    }
}

impl Sub for Point {
    type Output = u32;

    fn sub(self, other: Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Person = 1,
    Key = 2,
    Aim = 3,
    Wall = 4,
}

const PERSON: (u32, u32) = (5, 0);
const KEY: (u32, u32) = (4, 1);
const AIM: (u32, u32) = (4, 13);
const WALLS: [[(u32, u32); 2]; 5] = [
    [(2, 3), (5, 3)],
    [(0, 6), (3, 6)],
    [(0, 9), (2, 9)],
    [(4, 9), (5, 9)],
    [(2, 12), (5, 12)],
];
const MAX_CNT: i32 = 100;
const CRASH_WALL: f64 = -1000.0;
const GOAL_REWARD: f64 = 0.0;
const CAL_PERSON_KEY_DIS: bool = true;
const CAL_KEY_AIM_DIS: bool = true;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Game {
    width: u32,
    height: u32,
    state: ((u32, u32), (u32, u32)),
    next_state: ((u32, u32), (u32, u32)),
    game_over: bool,
    cnt: i32,
    room: Vec<Cell>,
    ai: QLearning,
    is_log: bool,
}

#[inline]
fn get_index(width: u32, row: u32, column: u32) -> usize {
    (row * width + column) as usize
}

impl Game {
    pub fn clear_room(&mut self) {
        self.room.iter_mut().for_each(|x| *x = Cell::Empty);
    }

    fn get_reward(&mut self) -> f64 {
        let mut reward = -1.0;
        if self.game_over {
            reward += CRASH_WALL;
        }

        if self.next_state.1 == AIM {
            self.game_over = true;
            if !CAL_KEY_AIM_DIS && !CAL_PERSON_KEY_DIS {
                reward += GOAL_REWARD;
            }
        }
        if CAL_PERSON_KEY_DIS {
            let person_pos = Point::new(self.next_state.0);
            let key_pos = Point::new(self.next_state.1);
            reward += -((person_pos - key_pos) as f64);
        }
        if CAL_KEY_AIM_DIS {
            let key_pos = Point::new(self.next_state.1);
            let aim_pos = Point::new(AIM);
            reward += -((key_pos - aim_pos) as f64);
        }
        reward
    }

    fn step(&mut self, action: Move) {
        let x = self.state.0 .0;
        let y = self.state.0 .1;

        let nx = x as i32 + MOVE[action as usize].0;
        let ny = y as i32 + MOVE[action as usize].1;
        if nx < 0 || nx >= self.height as i32 || ny < 0 || ny >= self.width as i32 {
            self.next_state = self.state;
            return;
        }

        let (nx, ny) = (nx as u32, ny as u32);
        self.next_state.0 = (nx, ny);
        let now_idx = get_index(self.width, nx as u32, ny as u32);
        if self.room[now_idx] == Cell::Wall {
            self.game_over = true;
            return;
        }

        if self.room[now_idx] == Cell::Key {
            let knx = nx as i32 + MOVE[action as usize].0;
            let kny = ny as i32 + MOVE[action as usize].1;
            if knx < 0 || knx >= self.height as i32 || kny < 0 || kny >= self.width as i32 {
                self.next_state = self.state;
                return;
            }
            let (knx, kny) = (knx as u32, kny as u32);
            self.next_state.1 = (knx, kny);
            let idx = get_index(self.width, knx, kny);
            if self.room[idx] == Cell::Wall {
                self.game_over = true;
                return;
            }
        }

        self.room[get_index(self.width, self.state.1 .0, self.state.1 .1)] = Cell::Empty;
        self.room[get_index(self.width, self.next_state.1 .0, self.next_state.1 .1)] = Cell::Key;
        self.room[get_index(self.width, self.state.0 .0, self.state.0 .1)] = Cell::Empty;
        self.room[get_index(self.width, self.next_state.0 .0, self.next_state.0 .1)] = Cell::Person;

        return;
    }
}

#[wasm_bindgen]
impl Game {
    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        let action = self.ai.take_action(self.state);

        self.step(action);
        let reward = self.get_reward();

        if self.is_log {
            log!("{} {:?} {} ", self.cnt, action, reward);
        }

        self.ai.train(action, self.state, self.next_state, reward);

        self.cnt += 1;
        if self.cnt >= MAX_CNT {
            self.game_over = true;
        }
        self.state = self.next_state;
    }

    pub fn new() -> Game {
        utils::set_panic_hook();
        let width = 14;
        let height = 6;

        let mut room = vec![Cell::Empty; height as usize * width as usize];
        room[get_index(width, PERSON.0, PERSON.1)] = Cell::Person;
        room[get_index(width, KEY.0, KEY.1)] = Cell::Key;
        room[get_index(width, AIM.0, AIM.1)] = Cell::Aim;
        for wall in WALLS {
            let y = wall[0].1;
            for x in wall[0].0..=wall[1].0 {
                room[get_index(width, x, y)] = Cell::Wall;
            }
        }

        Game {
            width,
            height,
            state: (PERSON, KEY),
            next_state: (PERSON, KEY),
            game_over: false,
            cnt: 0,
            room,
            ai: QLearning::new(),
            is_log: false,
        }
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn room(&self) -> *const Cell {
        self.room.as_ptr()
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn restart(&mut self) {
        self.clear_room();
        self.room[get_index(self.width, PERSON.0, PERSON.1)] = Cell::Person;
        self.room[get_index(self.width, KEY.0, KEY.1)] = Cell::Key;
        self.room[get_index(self.width, AIM.0, AIM.1)] = Cell::Aim;
        for wall in WALLS {
            let y = wall[0].1;
            for x in wall[0].0..=wall[1].0 {
                self.room[get_index(self.width, x, y)] = Cell::Wall;
            }
        }
        self.state = (PERSON, KEY);
        self.next_state = self.state;
        self.game_over = false;
        self.cnt = 0;
    }

    pub fn set_is_log(&mut self, v: bool) {
        self.is_log = v;
    }
}
