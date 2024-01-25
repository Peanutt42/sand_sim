use core::panic;

use minifb::{Window, WindowOptions, Key, KeyRepeat, MouseButton, MouseMode};

use sand_sim::{Simulation, Cell};

fn main() {
	if cfg!(debug_assertions) {
		println!("Running in debug build will be much slower as in a release build!");
	}

    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut window = Window::new(
        "Sand simulation",
        WIDTH, HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        })
        .unwrap_or_else(|e| panic!("Failed to create window: {e}"));

    let mut simulation = Simulation::new(WIDTH, HEIGHT);

    let mut screen: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() {
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            for y in 0..simulation.height {
                for x in 0..simulation.width {
                    simulation.grid[x + y * WIDTH] = Cell::Empty;
                }
            }
        }

        let mouse_left = window.get_mouse_down(MouseButton::Left);
        let mouse_right = window.get_mouse_down(MouseButton::Right);
        if mouse_left || mouse_right {
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let (window_width, window_height) = window.get_size();
                let mouse_x = (mouse_x * (WIDTH as f32 / window_width as f32)) as i32;
                let mouse_y = (mouse_y * (HEIGHT as f32 / window_height as f32)) as i32;
                let extend = if mouse_left { 15 } else { 5 };
                for y in mouse_y-extend..mouse_y+extend {
                    for x in mouse_x-extend..mouse_x+extend {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                            simulation.grid[x as usize + y as usize * WIDTH] = if mouse_left { Cell::Sand } else { Cell::Stone };
                        }
                    }
                }
            }
        }

        if !window.is_key_down(Key::Space) {
            simulation.update();
        }
        
        simulation.draw_to_buffer(&mut screen);

        if let Err(e) = window.update_with_buffer(&screen, WIDTH, HEIGHT) {
            println!("Failed to update window with buffer: {e}");
        }
    }
}
