
#[derive(Clone, PartialEq)]
pub enum PixelState {
	Empty,
	Sand,
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

	pub fn update(&mut self) {
		let mut updated_grid = vec![PixelState::Empty; self.width * self.height];

		for y in 0..self.height {
			for x in 0..self.width {
				match self.grid[x + y * self.width] {
					PixelState::Sand => {
						if y == self.height - 1 {
							updated_grid[x + y * self.width] = PixelState::Sand;
							continue;
						}

						let below = &self.grid[x + (y + 1) * self.width];
						let mut below_left: Option<&PixelState> = None;
						let mut below_right: Option<&PixelState> = None;
						if x > 0 {
							below_left = Some(&self.grid[x - 1 + (y + 1) * self.width]);
						}
						if x + 1 < self.width {
							below_right = Some(&self.grid[x + 1 + (y + 1) * self.width]);
						}
						if *below == PixelState::Empty {
							updated_grid[x + y * self.width] = PixelState::Empty;
							updated_grid[x + (y + 1) * self.width] = PixelState::Sand;
						}
						else if below_left.is_some() && *below_left.unwrap() == PixelState::Empty {
                            updated_grid[x - 1 + (y + 1) * self.width] = PixelState::Sand;
                        }
						else if below_right.is_some() && *below_right.unwrap() == PixelState::Empty {
                            updated_grid[x + 1 + (y + 1) * self.width] = PixelState::Sand;
                        }
						else {
							updated_grid[x + y * self.width] = PixelState::Sand;
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

		for y in 0..self.height {
			for x in 0..self.width {
				match self.grid[x + y * self.width] {
                    PixelState::Sand => {
                        buffer[x + y * self.width] = 0xFFFFFFFF;
                    },
                    _ => {
                        buffer[x + y * self.width] = 0;
					},
                }
			}
		}
	}
}