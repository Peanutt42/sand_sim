#[derive(Clone, Copy, PartialEq)]
pub enum PixelState {
	Empty,
	Sand,
	Stone,
}

pub struct Simulation {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<PixelState>,
}

impl Simulation {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			grid: vec![PixelState::Empty; width * height],
		}
    }

	fn swap(&mut self, a: usize, b: usize) {
		let tmp = self.grid[a].clone();
        self.grid[a] = self.grid[b];
        self.grid[b] = tmp;
	}

	fn is_empty(&self, index: usize) -> bool {
		if index >= self.grid.len() {
			return true;
		}
		self.grid[index] == PixelState::Empty
	}

	pub fn update_pixel(&mut self, x: usize, y: usize) {
		match self.grid[x + y * self.width] {
			PixelState::Sand => {
				if y == self.height - 1 {
					return;
				}

				let index = x + y * self.width;

				let below = index + self.width;
				let below_left = below - 1;
				let below_right = below + 1;

				if self.is_empty(below) {
					self.swap(index, below);
				}
				else if x > 0 && self.is_empty(below_left) {
					self.swap(index, below_left);
				}
				else if x + 1 < self.width && self.is_empty(below_right) {
					self.swap(index, below_right);
				}
			},
			_ => {},
		}
	}

	pub fn update(&mut self) {
		for y in (0..self.height).rev() {
			let left_to_right = y % 2 == 0;
			if left_to_right {
				for x in 0..self.width {
                    self.update_pixel(x, y);
                }
			}
			else {
				for x in (0..self.width).rev() {
					self.update_pixel(x, y);
				}
			}
		}
	}

	pub fn draw_to_buffer(&self, buffer: &mut Vec<u32>) {
		buffer.resize(self.width * self.height, 0);

		for y in 0..self.height {
			for x in 0..self.width {
				match self.grid[x + y * self.width] {
                    PixelState::Sand => {
                        buffer[x + y * self.width] = 0xFFFFFFFF;
                    },
					PixelState::Stone => {
                        buffer[x + y * self.width] = 0x999999FF;
					},
                    _ => {
                        buffer[x + y * self.width] = 0;
					},
                }
			}
		}
	}
}