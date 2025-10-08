use macroquad::{prelude::*, rand::RandomRange};

const SIZE: usize = 200;
const SQUARE_SIZE: f32 = 5.;
const DELAY: f64 = 0.2;

struct World {
    cells: Vec<u16>,
}

impl World {
    fn new() -> Self {
        World {
            cells: World::create_cells(),
        }
    }

    fn create_cells() -> Vec<u16> {
        let mut cells: Vec<u16> = Vec::new();

        for _ in 0..SIZE * SIZE {
            cells.push(World::random_binary());
        }

        // cells[0] = 1;
        return cells;
    }

    fn random_binary() -> u16 {
        let val = RandomRange::gen_range(0., 0.99);

        if val < 0.5 {
            return 1;
        } else {
            return 0;
        }
    }

    fn draw_cell(&self, i: usize, j: usize, is_alive: u16) {
        let color = if is_alive == 1 { WHITE } else { BLACK };
        draw_rectangle(
            i as f32 * SQUARE_SIZE,
            j as f32 * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            color,
        )
    }

    fn draw(&self) {
        for j in 0..SIZE {
            for i in 0..SIZE {
                let cell = self.cells[j * SIZE + i];
                self.draw_cell(i, j, cell);
            }
        }
    }

    fn count_neighbors(&self, i: usize, j: usize) -> u16 {
        let mut sum: u16 = 0;
        let x_from = i.saturating_sub(1);
        let x_to = (i + 2).min(SIZE);

        let y_from = j.saturating_sub(1);
        let y_to = (j + 2).min(SIZE);

        for y in y_from..y_to {
            for x in x_from..x_to {
                if x == i && y == j {
                    continue;
                }

                let value = self.cells[y * SIZE + x];
                sum += value;
            }
        }

        return sum;
    }

    fn get_copy(&self) -> Vec<u16> {
        let mut new: Vec<u16> = Vec::new();

        for e in &self.cells {
            new.push(e.clone());
        }

        return new;
    }

    fn next_state_cells(&mut self) {
        let mut copy = self.get_copy();
        // println!("Cells: {:?}", self.cells);

        for j in 0..SIZE {
            for i in 0..SIZE {
                let count = self.count_neighbors(i, j);
                let is_alive = self.cells[j * SIZE + i];
                let new_state = self.next_state_cell(count, is_alive);

                // println!("State {}, {}, {}, {}", i, j, count, new_state);
                copy[j * SIZE + i] = new_state;
            }
        }

        self.cells = copy;
    }

    fn next_state_cell(&self, n_count: u16, is_alive: u16) -> u16 {
        if is_alive == 1 {
            if n_count < 2 {
                return 0;
            }

            if n_count <= 3 {
                return 1;
            } else {
                return 0;
            }
        }

        if n_count == 3 {
            return 1;
        }

        return is_alive;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut world = World::new();
    let mut time = get_time();

    loop {
        clear_background(BLACK);

        world.draw();

        if get_time() - time > DELAY {
            world.next_state_cells();
            time = get_time();
        }

        next_frame().await
    }
}
