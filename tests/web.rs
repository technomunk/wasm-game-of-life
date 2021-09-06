//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_game_of_life;
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
	let mut universe = Universe::empty(6, 6);
	universe.place([(1,2), (2,3), (3,1), (3,2), (3,3)], 0, 0);
	universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
	let mut universe = Universe::empty(6, 6);
	universe.place([(2,1), (2,3), (3,2), (3,3), (4,2)], 0, 0);
	universe
}

#[wasm_bindgen_test]
pub fn test_trivial_tick() {
	let mut universe = Universe::empty(1, 1);
	universe.place([(0, 0)], 0, 0);
	universe.tick();

	assert_eq!(universe, Universe::empty(1, 1));
}

#[wasm_bindgen_test]
pub fn test_cross_tick() {
	let mut universe = Universe::empty(4, 4);
	universe.place([(1, 0), (1, 1), (1, 2)], 0, 0);

	let expected = {
		let mut universe = Universe::empty(4, 4);
		universe.place([(0, 1), (1, 1), (2, 1)], 0, 0);
		universe
	};

	universe.tick();
	assert_eq!(universe, expected)
}

#[wasm_bindgen_test]
pub fn test_spaceship_tick() {
	let mut input_universe = input_spaceship();
	input_universe.tick();

	let expected = expected_spaceship();
	assert_eq!(input_universe, expected);
}
