use macroquad::{prelude::*, rand::RandomRange};

const SIZE: usize = 200;
const SQUARE_SIZE: f32 = 5.;
const DELAY: f64 = 0.5;

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

        for _ in 0..SIZE {
            for _ in 0..SIZE {
                cells.push(World::random_binary());
            }
        }

        return cells;
    }

    fn random_binary() -> u16 {
        let rand = RandomRange::gen_range(0., 0.99);

        return if rand > 0.5 { 1 } else { 0 };
    }

    fn draw(&self) {
        self.draw_cells();
    }

    fn draw_cells(&self) {
        for j in 0..SIZE {
            for i in 0..SIZE {
                let is_alive = self.cells[j * SIZE + i];
                let color = if is_alive == 1 { WHITE } else { BLACK };

                draw_rectangle(
                    i as f32 * SQUARE_SIZE,
                    j as f32 * SQUARE_SIZE,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    color,
                )
            }
        }
    }

    fn get_copy(&self) -> Vec<u16> {
        let mut new_cells: Vec<u16> = Vec::new();
        for c in &self.cells {
            new_cells.push(c.clone());
        }

        return new_cells;
    }

    fn next_gen(&mut self) {
        let mut copy = self.get_copy();

        // println!("Cells: {:?}", self.cells);

        for j in 0..SIZE {
            for i in 0..SIZE {
                let is_alive = self.cells[j * SIZE + i];
                let count = self.neighbor_count(i, j);
                let state = World::get_next_state(count, is_alive);

                // println!(" {} {} {} {}", i, j, count, state);

                copy[j * SIZE + i] = state;
            }
        }

        self.cells = copy;
    }

    fn get_next_state(count: u16, is_alive: u16) -> u16 {
        if is_alive == 1 {
            if count < 2 {
                return 0;
            }

            if count <= 3 {
                return 1;
            }

            return 0;
        }

        if count == 3 {
            return 1;
        }

        return is_alive;
    }

    fn neighbor_count(&self, i: usize, j: usize) -> u16 {
        let i_from = i.saturating_sub(1);
        let i_to = (i + 2).min(SIZE);

        let j_from = j.saturating_sub(1);
        let j_to = (j + 2).min(SIZE);

        let mut sum = 0;
        for tj in j_from..j_to {
            for ti in i_from..i_to {
                if tj == j && ti == i {
                    continue;
                }

                let alive = self.cells[tj * SIZE + ti];

                sum = sum + alive;
            }
        }

        return sum;
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
            time = get_time();
            world.next_gen();
        }

        next_frame().await
    }
}
