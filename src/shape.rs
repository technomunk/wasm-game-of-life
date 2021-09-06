use js_sys::Math;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transformation {
	Identity,
	RotateLeft,
	RotateRight,
	Reflect,
}

impl Transformation {
	pub fn random() -> Self {
		match Math::random() {
			x if x < 0.25 => Self::Identity,
			x if x < 0.5 => Self::RotateLeft,
			x if x < 0.75 => Self::RotateRight,
			_ => Self::Reflect,
		}
	}
}

pub fn transform((x, y): (u32, u32), w: u32, h: u32, t: Transformation) -> (u32, u32) {
	match t {
		Transformation::Identity => (x, y),
		Transformation::RotateRight => (h - y - 1, x),
		Transformation::Reflect => (w - x - 1, h - 1 - y),
		Transformation::RotateLeft => (y, w - x - 1)
	}
}

pub const GLIDER: &[(u32, u32)] = &[(0, 0), (1, 0), (0, 1), (2, 1), (0, 2)];
