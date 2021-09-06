use std::fmt;

mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math;

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
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

// Private helper methods
impl Universe {
	/// Get the index of the cell at provided coordinates.
	///
	/// Note: emulates a wrapping universe by using modulus.
	fn idx(&self, x: u32, y: u32) -> usize {
		(y % self.height * self.width + x % self.width) as usize
	}

	/// Get the number of living neighbors around the provided cell.
	fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
		const OFFSETS: [u32; 3] = [u32::MAX, 0, 1];
		let mut count = 0;
		for xo in OFFSETS {
			for yo in OFFSETS {
				if xo == 0 && yo == 0 {
					continue;
				}

				count += self.cells[self.idx(x+xo, y+yo)] as u8;
			}
		}

		count
	}
}

// Public methods
#[wasm_bindgen]
impl Universe {
	/// Get the cell state at provided coordinates.
	///
	/// True means 'alive', false means 'dead'.
	pub fn tick(&mut self) {
		let mut next = self.cells.clone();
		for y in 0..self.height {
			for x in 0..self.width {
				let idx = self.idx(x, y);
				let cell = self.cells[idx];
				let live_neighbor_count = self.live_neighbor_count(x, y);

				next[idx] = match (cell, live_neighbor_count) {
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					(Cell::Dead, 3) => Cell::Alive,
					(otherwise, _) => otherwise, // in other cases cell state remains the same
				}
			}
		}

		self.cells = next;
	}

	/// Initialize a new universe with an interesting pattern.
	pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|_| if Math::random() > 0.5 {
				Cell::Alive
			} else {
				Cell::Dead
			})
            .collect();

        Self {
            width,
            height,
            cells,
        }
	}

	/// Present the universe to the viewer.
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Get the width of the universe in cells.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the universe in cells.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the pointer to cell data in the universe.
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
