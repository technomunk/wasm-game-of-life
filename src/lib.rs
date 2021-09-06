mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Bit-dense storage for cells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitStore(Vec<u8>);

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

	/// Set cells at provided coordinates to alive state.
	pub fn set_cells<'a, T: IntoIterator<Item = &'a (u32, u32)>>(&mut self, cells: T) {
		for &(x, y) in cells {
			self.cells.set(self.idx(x, y), true)
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
}

impl BitStore {
	/// Test whether the provided bit is set.
	pub fn get(&self, idx: usize) -> bool {
		let mask = 1 << (idx % 8);
		self.0[idx/8] & mask == mask 
	}

	/// Set the provided bit to given state.
	pub fn set(&mut self, idx: usize, val: bool) {
		let mask = 1 << (idx % 8);
		if val {
			self.0[idx/8] |= mask
		} else {
			self.0[idx/8] &= !mask
		}
	}

	/// Get the number of bytes the data occupies.
	pub fn size(&self) -> usize {
		self.0.len()
	}

	pub fn as_ptr(&self) -> *const u8 {
		self.0.as_ptr()
	}

	/// Generate a randomly filled BitStore with at least the provided bit count.
	pub fn random(length: usize) -> Self {
		let rounding = (length % 8 != 0) as usize;
		let max = length / 8 + rounding;
		// NOTE: a more efficient variant would be to invoke Math::random() for every 50ish bits
		// as it's the most computationally expensive operation during initialization.
		Self((0..max)
			.map(|_| (Math::random() * 256.0) as u8)
			.collect()
		)
	}

	/// Create an empty bitstore with at least provided bit count.
	pub fn empty(length: usize) -> Self {
		let rounding = (length % 8 != 0) as usize;
		let max = length / 8 + rounding;
		Self((0..max).map(|_| 0).collect())
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
	fn test_bitstore() {
		let mut store = BitStore::empty(2);
	
		assert_eq!(&store.0, &[0]);
	
		store.set(0, true);
		assert_eq!(&store.0, &[1]);
	
		store.set(1, true);
		assert_eq!(&store.0, &[3]);
	
		store.set(0, false);
		assert_eq!(&store.0, &[2]);
	}
	
	#[test]
	fn test_neighbor_count() {
		// leave an empty row and column to avoid wrapping artifacts
		let mut universe = Universe::empty(4, 4);
		universe.set_cells(&[(1, 0), (1, 1), (1, 2)]);

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
		universe.set_cells(CELLS);

		for &(x, y) in CELLS {
			assert!(universe.cells.get(universe.idx(x, y)));
		}
	}
}
