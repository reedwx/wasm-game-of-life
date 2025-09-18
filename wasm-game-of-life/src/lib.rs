mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_cell_count(&self, row: u32, col: u32) -> u8 {
        let mut living_cells: u8 = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                living_cells += self.cells[idx] as u8;
            }
        }
        return living_cells;
    }

    /// Note: I played around with this for a bit to see if I could get away
    /// with removing the initial .clone() call, but it's not practical b/c
    /// you have to read from the old Universe to determine how to set the new
    /// one, creating a situation where both immutable + mutable borrows are
    /// required. You can add a "next" buffer in the Universe struct, alter that,
    /// and then set the cur_cells pointer to next at the end of tick(), but that sorta
    ///  defeats the purpose of the exercise
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                //determine if each cell is alive or dead in next iteration
                let live_count = self.live_cell_count(row, col);
                let status: Cell = self.cells[self.get_index(row, col)];
                let next_status = match (live_count, status) {
                    (x, Cell::Alive) if x < 2 => Cell::Dead,
                    (x, Cell::Alive) if (x == 2 || x == 3) => Cell::Alive,
                    (x, Cell::Alive) if x > 3 => Cell::Dead,
                    (3, Cell::Dead) => Cell::Alive,
                    (_, otherwise) => otherwise,
                };
                next[self.get_index(row, col)] = next_status;
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

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
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    //set width + set cells as dead
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    //set height + set cells as dead
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
}

/// separate impl block required since Rust-generated WASM functions
/// cannot return borrowed references
impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    //Set provided cells as alive within grid
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter() {
            let index = self.get_index(*row, *col);
            self.cells[index] = Cell::Alive;
        }
    }
}

use std::fmt;

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
