
#[derive(Clone, PartialEq)]
pub enum PixelState {
	Empty,
	Sand,
}

pub struct Simulation {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<Vec<PixelState>>,
}

impl Simulation {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			grid: vec![vec![PixelState::Empty; height]; width],
		}
    }

	pub fn update(&mut self) {
		let mut updated_grid = vec![vec![PixelState::Empty; self.height]; self.width];

		for col in 0..self.width {
			for row in 0..self.height {
				match self.grid[col][row] {
					PixelState::Sand => {
						if row == self.height - 1 {
							updated_grid[col][row] = PixelState::Sand;
							continue;
						}

						let below = &self.grid[col][row + 1];
						let mut below_left: Option<&PixelState> = None;
						let mut below_right: Option<&PixelState> = None;
						if col > 0 {
							below_left = Some(&self.grid[col - 1][row + 1]);
						}
						if col + 1 < self.width {
							below_right = Some(&self.grid[col + 1][row + 1]);
						}
						if *below == PixelState::Empty {
							updated_grid[col][row] = PixelState::Empty;
							updated_grid[col][row + 1] = PixelState::Sand;
						}
						else if below_left.is_some() && *below_left.unwrap() == PixelState::Empty {
                            updated_grid[col - 1][row + 1] = PixelState::Sand;
                        }
						else if below_right.is_some() && *below_right.unwrap() == PixelState::Empty {
                            updated_grid[col + 1][row + 1] = PixelState::Sand;
                        }
						else {
							updated_grid[col][row] = PixelState::Sand;
						}
					},
					_ => {},
				}
			}
		}

		self.grid = updated_grid;
	}

	pub fn draw_to_buffer(&self, buffer: &mut Vec<u32>) {
		buffer.resize(self.width * self.height, 0);

		for col in 0..self.width {
			for row in 0..self.height {
				match self.grid[col][row] {
                    PixelState::Sand => {
                        buffer[col + row * self.width] = 0xFFFFFFFF;
                    },
                    _ => {
                        buffer[col + row * self.width] = 0;
					},
                }
			}
		}
	}
}