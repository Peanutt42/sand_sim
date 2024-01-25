use core::panic;

use minifb::{Window, WindowOptions, Key, KeyRepeat, MouseButton, MouseMode};

use sand_sim::{Simulation, Cell};

fn main() {
	if cfg!(debug_assertions) {
		println!("Running in debug build will be much slower as in a release build!");
	}

    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    let mut window = Window::new(
        "Sand simulation",
        WIDTH, HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        })
        .unwrap_or_else(|e| panic!("Failed to create window: {e}"));

    let mut simulation = Simulation::new(WIDTH, HEIGHT);
    let mut selected_cell_type = Cell::Sand;
    let mut brush_extend = 5;

    let mut screen: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() {
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            for y in 0..simulation.height {
                for x in 0..simulation.width {
                    simulation.grid[x + y * WIDTH] = Cell::Empty;
                }
            }
        }

        if let Some((_, scroll)) = window.get_scroll_wheel() {
            if scroll > 0.0 {
                brush_extend += 5;
            }
            else if scroll < 0.0 {
                brush_extend -= 5;
            }
        }

        if window.is_key_down(Key::Key1) {
            selected_cell_type = Cell::Sand;
        }
        if window.is_key_down(Key::Key2) {
            selected_cell_type = Cell::Stone;
        }

        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp);
        let (window_width, window_height) = window.get_size();

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mouse_x, mouse_y)) = mouse_pos {
                let mouse_x = (mouse_x * (WIDTH as f32 / window_width as f32)) as i32;
                let mouse_y = (mouse_y * (HEIGHT as f32 / window_height as f32)) as i32;
                simulation.set_box(mouse_x, mouse_y, brush_extend, selected_cell_type);
            }
        }
        if window.get_mouse_down(MouseButton::Right) {
            if let Some((mouse_x, mouse_y)) = mouse_pos {
                let mouse_x = (mouse_x * (WIDTH as f32 / window_width as f32)) as i32;
                let mouse_y = (mouse_y * (HEIGHT as f32 / window_height as f32)) as i32;
                simulation.set_box(mouse_x, mouse_y, brush_extend, Cell::Empty);
            }
        }

        if !window.is_key_down(Key::Space) {
            simulation.update();
        }
        
        simulation.draw_to_buffer(&mut screen);

        if let Some((mouse_x, mouse_y)) = mouse_pos {
            let mouse_x = (mouse_x * (WIDTH as f32 / window_width as f32)) as i32;
            let mouse_y = (mouse_y * (HEIGHT as f32 / window_height as f32)) as i32;
            for y in mouse_x-brush_extend..mouse_x+brush_extend {
                for x in mouse_y-brush_extend..mouse_y+brush_extend {
                    if x >= 0 && x < window_width as i32 && y >= 0 && y < window_height as i32 {
                        screen[x as usize * window_height + y as usize] = 0xFFFFFFFF;
                    }
                }
            }
        }

        if let Err(e) = window.update_with_buffer(&screen, WIDTH, HEIGHT) {
            println!("Failed to update window with buffer: {e}");
        }
    }
}
