use rand::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
	Empty,
	Sand,
	Stone,
}

#[derive(Clone, Copy)]
struct CellMove {
	destination: i32,
	source: i32,
}

pub struct Simulation {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<Cell>,
}

impl Simulation {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			grid: vec![Cell::Empty; width * height],
		}
    }

	fn is_empty(&self, index: usize) -> bool {
		if index >= self.grid.len() {
			return true;
		}
		self.grid[index] == Cell::Empty
	}

	fn update_pixel(&self, x: usize, y: usize) -> Option<CellMove> {
		match self.grid[x + y * self.width] {
			Cell::Sand => {
				if y == self.height - 1 {
					return None;
				}

				let index = x + y * self.width;

				let below = index + self.width;
				let below_left = below - 1;
				let can_go_below_left = x > 0 && self.is_empty(below_left);
				let below_right = below + 1;
				let can_go_below_right = x + 1 < self.width && self.is_empty(below_right);

				if self.is_empty(below) {
					Some(CellMove { destination: below as i32, source: index as i32 })
				}
				else if can_go_below_left && can_go_below_right {
					let destination = if rand::random::<bool>() { below_left } else { below_right } as i32;
					Some(CellMove { destination, source: index as i32 })
				}
				else if can_go_below_left {
					Some(CellMove { destination: below_left as i32, source: index as i32 })
				}
				else if can_go_below_right {
					Some(CellMove { destination: below_right as i32, source: index as i32 })
				}
				else {
					None
				}
			},
			_ => None,
		}
	}

	pub fn update(&mut self) {
		let mut changes: Vec<CellMove> = Vec::new();

		for y in (0..self.height).rev() {
			let left_to_right = y % 2 == 0;
			if left_to_right {
				for x in 0..self.width {
                    if let Some(cell_move) = self.update_pixel(x, y) {
						changes.push(cell_move);
					}
                }
			}
			else {
				for x in (0..self.width).rev() {
					if let Some(cell_move) = self.update_pixel(x, y) {
						changes.push(cell_move);
					}
				}
			}
		}

		self.commit_changes(changes);
	}

	fn commit_changes(&mut self, mut changes: Vec<CellMove>) {
		// removes moves that are not valid
		for mut i in 0..changes.len() {
			if self.grid[changes[i].destination as usize] != Cell::Empty {
				changes[i] = *changes.last().unwrap();
				changes.pop();
				i -= 1;
			}
		}

		changes.sort_by(|a, b| a.destination.cmp(&b.destination));

		let mut iprev = 0;
		changes.push(CellMove{ destination: -1, source: -1 });
		for i in 0..changes.len()-1 {
			if changes[i + 1].destination != changes[i].destination {
				let rand = iprev + (rand::random::<usize>() % (i-iprev+1));
				let dst = changes[rand].destination.clone();
				let src = changes[rand].source.clone();
				self.grid[dst as usize] = self.grid[src as usize];
				self.grid[src as usize] = Cell::Empty;

				iprev = i + 1;
			}
		}
	}

	pub fn draw_to_buffer(&self, buffer: &mut Vec<u32>) {
		buffer.resize(self.width * self.height, 0);

		for y in 0..self.height {
			for x in 0..self.width {
				match self.grid[x + y * self.width] {
                    Cell::Sand => {
                        buffer[x + y * self.width] = 0xFFFFFFFF;
                    },
					Cell::Stone => {
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