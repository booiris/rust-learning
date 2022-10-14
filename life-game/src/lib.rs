mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells {
            let idx = self.get_index(*row, *col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = vec![Cell::Dead; self.cells.len()];

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // 规则 1: 任何少于两个邻居的活细胞死亡，就好像是由于人口不足造成的一样。.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // 规则 2: 任何一个有两个或三个邻居的活体细胞都能传到下一代。.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // 规则 3: 任何居住着三个以上邻居的活细胞都会死亡，就好像是由于人口过剩。.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // 规则 4:任何一个只有三个相邻的活细胞的死细胞都会变成活细胞，就像通过繁殖一样。.
                    (Cell::Dead, 3) => Cell::Alive,
                    // 所有其他单元格保持相同状态。
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );
            }
        }

        // log!("    it becomes {:?}", next);

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 90;
        let height = 90;

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.2 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn restart(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                if js_sys::Math::random() < 0.2 {
                    self.cells[idx] = Cell::Alive;
                } else {
                    self.cells[idx] = Cell::Dead
                }
            }
        }
    }
}

use web_sys::console;
pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
