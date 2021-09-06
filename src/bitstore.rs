use js_sys::Math;

/// Bit-dense storage for cells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitStore(Vec<u8>);

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
