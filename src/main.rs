use std::usize;
use minifb::{WindowOptions, Window, Key};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const SAND_SIZE: u32 = 4;

struct SandParticle {
    xcoord: u32,
    ycoord: u32,
}

impl SandParticle {
    // Function to handle the particle falling
    fn fall(&mut self) {
        self.ycoord += 1;
    }
}

fn main() {
    // Creating the image buffer (flattened matrix so index Y first on editing)
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Initialising the window
    let mut window = match Window::new("Falling Sand", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default()) {
        Ok(window) => window,
        Err(err) => {
            println!("Failed to initialise window: {}", err);
            return;
        },
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);

        // Check if the mouse is down
        if window.get_mouse_down(minifb::MouseButton::Left) {
            if let Some(coords) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                println!("Mouse pressed at: {}, {}", coords.0, coords.1);
            }
        }

        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}

// This function updates the buffered image with the new positions of particles
fn update_buffer(mut buffer: &mut Vec<u32>) {
    
}
