mod utils;

//extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
use byteorder::BigEndian;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    automata_num: i8,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        // Initialize first row with only the mid cell alive
        for cell in next[0..self.width].iter() {
            let mut mid_point = self.width/2;
            if cell == mid_point {
                cell[mid_point] = Cell::Alive;
            } else {
                cell = Cell::Dead;
            }
        }

        // iterate and change rest of Vector data
        for row in 1..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                //let alive_or_dead = self.check_prev_row(idx);

                // initialize neighbors
                //  neighbors is a 3 member array to represent the above row neighbors
                    //  ◻◻◻
                    //   ◻
                let mut neighbors = [i8,3];

                    //  ◻◼◻
                    //   ◻
                neighbors[1] = self.get_index(row-1, col);

                    //  ◼◻◻
                    //   ◻
                if col == 0 {  // left corner case
                    neighbors[0] = self.get_index(row-1, self.width);
                } else {
                    neighbors[0] = self.get_index(row-1, col-1);
                }

                    //  ◻◻◼
                    //   ◻
                if col == self.width {  // right corner case
                    neighbors[2] = self.get_index(row-1, 0);
                } else {
                    neighbors[2] = self.get_index(row-1, col-1);
                }

                // convert i8 into cellular automata logic
                let mut binary_array = self.i8_to_binary_arr(self.automata_num);




                bool
            }
        }

/*
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                //let live_neighbors = self.live_neighbor_count(row, col);
                let aliveOrDead = self.check_prev_neighbors(idx);

                /*
                // automata rules
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };*/

                let next_cell = match (cell, &self) {

                };

                
                next[idx] = next_cell;
            }
        }*/
        self.cells = next;
    }
 
    // return the location of a certain cell
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    // convert i8 to an array to represent byte conversion
    fn i8_to_binary_arr(&self, mut num: i8) -> [i8;8] {
        let mut bin_arr = [i8;8];
        let mut idx = 0;
        while num > 0 {
            binary[idx] = num%2;
            idx += 1;
            num = num/2
        }
        bin_arr
    }

    /*
    fn check_prev_row (&self, index: usize) -> bool {
        let mut neighbors = [i8,3];
        let mut current_cell = cells[index];
        neighbors[0] = self.get_index(row-1, col-1);
        bool
    } */

    /*
// count cells of neighboring cells
fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height-1, 0, 1].iter().cloned() {
        for delta_col in [self.width-1, 0, 1].iter().cloned() {
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
} */

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let automata_num = 1;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }

            })
            .collect();


        Universe {
            width,
            height,
            automata_num,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

