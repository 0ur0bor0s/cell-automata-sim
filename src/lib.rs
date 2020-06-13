mod utils;

//extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::collections::HashMap;
use crate::utils::set_panic_hook;
//use byteorder::BigEndian;

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
    width: usize,
    height: usize,
    automata_num: u8,
    cells: Vec<Cell>,
    automata_rule_map: HashMap<[bool; 3], bool>,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {

        set_panic_hook();

        let mut next = self.cells.clone();

        // Initialize first row with only the mid cell alive
        let mid_point: usize = self.width / 2;
        for (i, cell) in next[0..self.width-1].iter_mut().enumerate() {
            if i == mid_point {
                *cell = Cell::Alive;
            } else {
                *cell = Cell::Dead;
            }
        }


        // iterate and change rest of Vector data
        for row in 1..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                //let cell = self.cells[idx];
                //let alive_or_dead = self.check_prev_row(idx);

                // initialize neighbors
                //  neighbors is a 3 member array to represent the above row neighbors
                    //  ◻◻◻
                    //   ◻
                let mut neighbors: [bool; 3] = [false; 3];

                    //  ◻◼◻
                    //   ◻
                //neighbors[1] = self.get_index(row-1, col);
                neighbors[1] = self.get_cell_life(row-1, col);

                    //  ◼◻◻
                    //   ◻
                if col == 0 {  // left corner case
                    neighbors[0] = self.get_cell_life(row-1, self.width);
                } else {
                    neighbors[0] = self.get_cell_life(row-1, col-1);
                }

                    //  ◻◻◼
                    //   ◻
                if col == self.width {  // right corner case
                    neighbors[2] = self.get_cell_life(row-1, 0);
                } else {
                    neighbors[2] = self.get_cell_life(row-1, col+1);
                }

                // Draw next cell based off of automata rule map
                //let next_cell = Universe::dead_or_alive(&neighbors, neighbor_rules, binary_array);
                let mut next_cell = Cell::Dead;
                if *self.automata_rule_map.get(&neighbors).unwrap() == true {
                    next_cell = Cell::Alive;
                }

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
 
    // return the location of a certain cell
    fn get_index(&self, row: usize, col: usize) -> usize {
        ((row * self.width) + col) as usize
    }

    // return a reference to a certain cell
    fn get_cell_life(&self, row: usize, column: usize) -> bool {
        let index = self.get_index(row, column);
        let cell = self.cells[index];
        return match cell {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }

    // convert i8 to an array to represent byte conversion
    fn u8_to_binary_arr(mut num: u8) -> [bool; 8] {
        let mut bin_arr: [bool; 8] = [false; 8];
        let mut idx = 0;
        while num > 0 {
            // 1 == true, 0 == false
            if num % 2 == 1 {
                bin_arr[idx] = true;
            } else {
                bin_arr[idx] = false;
            }
            idx += 1;
            num = num / 2
        }
        bin_arr
    }

    pub fn new() -> Universe {
        let width = 256;
        let height = 256;
        let automata_num = 124;
        // place neighbor rules in array
        let neighbor_rules = [[true,  true,  true ],
                                         [true,  true,  false],
                                         [true,  false, true ],
                                         [true,  false, false],
                                         [false, true,  true ],
                                         [false, true,  false],
                                         [false, false, true ],
                                         [false, false, false]];

        // convert i8 into cellular automata logic
        let binary_array = Universe::u8_to_binary_arr(automata_num);

        // map binary array to corresponding neighbor values
        // binary array index:          0   1   2   3   4   5   6   7
        // neighbor rules:             ◼◼◼ ◼◼◻ ◼◻◼ ◼◻◻ ◻◼◼ ◻◼◻ ◻◻◼ ◻◻◻
        // example binary array (90):   ◻   ◼   ◻   ◼   ◼   ◻   ◼   ◻
        let automata_rule_map: HashMap<[bool; 3], bool> =
            neighbor_rules.iter().cloned().zip(binary_array.iter().cloned()).collect();

        let cells = (0..width * height)
            .map(|i| {
                if i % 3 == 0 || i % 5 == 0 {
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
            automata_rule_map,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
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

