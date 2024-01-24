use core::panic;

use minifb::{Window, WindowOptions, Key, KeyRepeat, MouseButton, MouseMode};

mod simulation;
use simulation::{Simulation, PixelState};

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
            resize: false,
            ..WindowOptions::default()
        })
        .unwrap_or_else(|e| panic!("Failed to create window: {e}"));

    let mut simulation = Simulation::new(WIDTH, HEIGHT);

    let mut screen: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() {
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            for y in 0..simulation.height {
                for x in 0..simulation.width {
                    simulation.grid[x + y * WIDTH] = PixelState::Empty;
                }
            }
        }

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let mouse_x = mouse_x as i32;
                let mouse_y = mouse_y as i32;
                for y in mouse_y-4..mouse_y+4 {
                    for x in mouse_x-4..mouse_x+4 {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                            simulation.grid[x as usize + y as usize * WIDTH] = PixelState::Sand;
                        }
                    }
                }
            }
        }
        if window.get_mouse_down(MouseButton::Right) {
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let mouse_x = mouse_x as i32;
                let mouse_y = mouse_y as i32;
                for y in mouse_y-4..mouse_y+4 {
                    for x in mouse_x-4..mouse_x+4 {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                            simulation.grid[x as usize + y as usize * WIDTH] = PixelState::Stone;
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
