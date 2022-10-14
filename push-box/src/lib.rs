#![allow(unused_macros)]
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[wasm_bindgen]
pub struct Game {
    width: u32,
    height: u32,
    person_pos: (u32, u32),
    game_over: bool,
    cnt: i32,
    room: Vec<Cell>,
}

fn get_index(width: u32, row: u32, column: u32) -> usize {
    (row * width + column) as usize
}

impl Game {
    pub fn get_room(&self) -> &[Cell] {
        &self.room
    }

    pub fn clear_room(&mut self) {
        self.room.iter_mut().for_each(|x| *x = Cell::Empty);
    }
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
const MOVE: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const MAX_CNT: i32 = 200;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
impl Game {
    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }
        self.cnt += 1;
        if self.cnt > MAX_CNT {
            self.game_over = true;
            log!("game over!");
            return;
        }

        let x = self.person_pos.0;
        let y = self.person_pos.1;
        let move_index;
        match js_sys::Math::random() {
            x if x < 0.2 => move_index = 0,
            x if x < 0.4 => move_index = 1,
            x if x < 0.6 => move_index = 2,
            _ => move_index = 3,
        }
        let nx = x as i32 + MOVE[move_index].0;
        let ny = y as i32 + MOVE[move_index].1;
        if nx < 0 || nx >= self.height as i32 || ny < 0 || ny >= self.width as i32 {
            return;
        }
        let (nx, ny) = (nx as u32, ny as u32);
        let idx = get_index(self.width, nx, ny);
        if self.room[idx] != Cell::Empty && self.room[idx] != Cell::Key {
            return;
        }

        if self.room[idx] == Cell::Key {
            let knx = nx as i32 + MOVE[move_index].0;
            let kny = ny as i32 + MOVE[move_index].1;
            if knx < 0 || knx >= self.height as i32 || kny < 0 || kny >= self.width as i32 {
                return;
            }
            let idx = get_index(self.width, knx as u32, kny as u32);
            if self.room[idx] != Cell::Empty {
                return;
            }
            let idx = get_index(self.width, nx as u32, ny as u32);
            self.room[idx] = Cell::Empty;
            let idx = get_index(self.width, knx as u32, kny as u32);
            self.room[idx] = Cell::Key;
        }
        
        let idx = get_index(self.width, x as u32, y as u32);
        self.room[idx] = Cell::Empty;
        let idx = get_index(self.width, nx as u32, ny as u32);
        self.room[idx] = Cell::Person;
        self.person_pos = (nx as u32, ny as u32);
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
            person_pos: PERSON,
            game_over: false,
            cnt: 0,
            room,
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
        self.person_pos = PERSON;
        self.game_over = false;
    }
}
