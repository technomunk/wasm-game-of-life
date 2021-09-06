mod utils;
mod bitstore;
mod shape;

use bitstore::BitStore;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub struct Universe {
	width: u32,
	height: u32,
	cells: BitStore,
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
	fn live_neighbor_count(&self, x: u32, y: u32) -> u32 {
		let mut count = 0;
		for yo in [self.height - 1, 0, 1] {
			for xo in [self.width - 1, 0, 1] {
				if xo == 0 && yo == 0 {
					continue;
				}

				let idx = self.idx(x.wrapping_add(xo), y.wrapping_add(yo));
				count += self.cells.get(idx) as u32;
			}
		}

		count
	}

	/// Get the cells of the universe.
	pub fn cells(&self) -> &BitStore {
		&self.cells
	}

	pub fn place<T>(&mut self, cells: T, xo: u32, yo: u32)
	where T: IntoIterator<Item = (u32, u32)> {
		for (x, y) in cells {
			self.cells.set(self.idx(x.wrapping_add(xo), y.wrapping_add(yo)), true)
		}
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
				let live_neighbor_count = self.live_neighbor_count(x, y);

				match (self.cells.get(idx), live_neighbor_count) {
					(true, x) if x < 2 => next.set(idx, false),
					(true, x) if x > 3 => next.set(idx, false),
					(false, 3) => next.set(idx, true),
					_ => (), // in other cases cell state remains the same
				}
			}
		}

		self.cells = next;
	}

	/// Create an empty universe.
	pub fn empty(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			cells: BitStore::empty((width*height) as usize)
		}
	}

	/// Initialize a new universe with an interesting pattern.
	pub fn random(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: BitStore::random((width*height) as usize),
        }
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
    pub fn cells_ptr(&self) -> *const u8 {
        self.cells.as_ptr()
    }

	pub fn cells_size(&self) -> usize {
		self.cells.size()
	}

	/// Toggle provided cell.
	pub fn toggle(&mut self, x: u32, y: u32) {
		let idx = self.idx(x, y);
		self.cells.set(idx, !self.cells.get(idx));
	}

	/// Spawn a randomly transformed glider at provided coordinates.
	pub fn spawn_glider(&mut self, x: u32, y: u32) {
		let tr = shape::Transformation::random();
		let cells = shape::GLIDER.iter()
			.map(|cell| shape::transform(*cell, 3, 3, tr));
		self.place(cells, x - 1, y - 1);
	}
}


impl std::fmt::Display for Universe {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for y in 0..self.height {
			for x in 0..self.width {
				if self.cells.get(self.idx(x, y)) {
					write!(f, "X")?;
				} else {
					write!(f, "-")?;
				}
			}
			write!(f, "\n")?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn test_neighbor_count() {
		// leave an empty row and column to avoid wrapping artifacts
		let mut universe = Universe::empty(4, 4);
		universe.place([(1, 0), (1, 1), (1, 2)], 0, 0);

		assert_eq!(universe.live_neighbor_count(0, 0), 2);
		assert_eq!(universe.live_neighbor_count(1, 0), 1);
		assert_eq!(universe.live_neighbor_count(2, 0), 2);
		
		assert_eq!(universe.live_neighbor_count(0, 1), 3);
		assert_eq!(universe.live_neighbor_count(1, 1), 2);
		assert_eq!(universe.live_neighbor_count(2, 1), 3);
		
		assert_eq!(universe.live_neighbor_count(0, 2), 2);
		assert_eq!(universe.live_neighbor_count(1, 2), 1);
		assert_eq!(universe.live_neighbor_count(2, 2), 2);

		// check wrapping neighbors
		assert_eq!(universe.live_neighbor_count(1, 3), 2);
	}

	#[test]
	fn test_get_cell() {
		const CELLS: &[(u32, u32)] = &[(1,2), (2,3), (3,1), (3,2), (3,3)];
		let mut universe = Universe::empty(6, 6);
		universe.place(CELLS.iter().copied(), 0, 0);

		for &(x, y) in CELLS {
			assert!(universe.cells.get(universe.idx(x, y)));
		}
	}
}
