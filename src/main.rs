use std::usize;
use minifb::{WindowOptions, Window, Key};
use rand::Rng;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const SAND_SIZE: u32 = 4;

struct SandParticle {
    xcoord: u32,
    ycoord: u32,
}

impl SandParticle {
    // Instantiates struct and adds position to grid
    fn new(grid: &mut Grid, xcoord: u32, ycoord: u32) -> Self {
        grid.occupy_cell(xcoord as usize, ycoord as usize);
        SandParticle { xcoord, ycoord }
    }

    // Function to handle the particle falling
    fn fall(&mut self, grid: &mut Grid) {
        // Preferable: true
        let bounded: bool = self.ycoord <= WINDOW_HEIGHT as u32 - 2;

        // Preferable: false (means the spot is empty)
        //let below: bool = coords.contains(&(self.xcoord, self.ycoord + 1));
        let below: bool = grid.cells[self.ycoord as usize + 1][self.xcoord as usize].is_occupied;
        //let bleft: bool = coords.contains(&(self.xcoord - 1, self.ycoord + 1));
        //let bright: bool = coords.contains(&(self.xcoord + 1, self.ycoord + 1));
        //let bboth: bool = bleft && bright;

        if bounded && !below {
            grid.leave_cell(self.xcoord as usize, self.xcoord as usize);
            self.ycoord += 1;
            grid.occupy_cell(self.xcoord as usize, self.ycoord as usize);
        }
        //else {
        //    if !bboth && below {
        //        let mut rng = rand::thread_rng();
        //        let random = rng.gen_range(1..101);
        //        if random > 50 {
        //            self.ycoord += 1;
        //            self.xcoord += 1;
        //            return;
        //        }
        //        self.ycoord += 1;
        //        self.xcoord -= 1;
        //    }
        //    else if !bleft && below {
        //        self.ycoord += 1;
        //        self.xcoord -= 1;
        //    }
        //    else if !bright && below {
        //        self.ycoord += 1;
        //        self.xcoord += 1;
        //    }
        //}
    }
}

struct Particles {
    particles: Vec<SandParticle>,
}

#[derive(Clone)]
struct GridCell {
    is_occupied: bool,
}

struct Grid {
    cells: Vec<Vec<GridCell>>,
}

impl Grid {
    fn new() -> Self {
        let cells = vec![vec![GridCell { is_occupied: false }; WINDOW_WIDTH]; WINDOW_HEIGHT];
        Grid { cells }
    }

    fn occupy_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].is_occupied = true;
    }

    fn leave_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].is_occupied = false;
    }
}

fn main() {
    // Creating the image buffer (flattened matrix so index Y first on editing)
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut part_vec = Particles{ particles: Vec::new() };

    let mut grid: Grid = Grid::new();

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
                part_vec.particles.push(SandParticle::new(&mut grid, coords.0 as u32, coords.1 as u32));
            }
        }

        physics_step(&mut part_vec, &mut grid);

        // Drawing the new frame
        buffer = new_frame(&mut part_vec);

        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}

// Allows for particles to fall
fn physics_step(part_vec: &mut Particles, grid: &mut Grid) {
    for particle in part_vec.particles.iter_mut() {
        particle.fall(grid);
    }
}

// This function returns the buffer for the new frame, given the new positions of particles
fn new_frame(part_vec: &Particles) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    for particle in &part_vec.particles {
        let x: usize = particle.xcoord as usize;
        let y: usize = particle.ycoord as usize;

        // Calculate the direct index in the buffer
        let overall_index = y * WINDOW_WIDTH + x;

        // Mark the particle's position in the buffer
        if overall_index < buffer.len() {
            buffer[overall_index] = 0xFFFFFF;
        }
    }

    buffer
}
