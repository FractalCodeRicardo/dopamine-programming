use macroquad::{prelude::*, rand::RandomRange};

const SIZE: usize = 10;
const SQUARE_SIZE: f32 = 10.;

struct World {
    cells: Vec<Vec<u16>>,
}

impl World {
    fn new() -> Self {
        World {
            cells: World::create_cells(),
        }
    }

    fn create_cells() -> Vec<Vec<u16>> {
        let mut cells: Vec<Vec<u16>> = Vec::new();

        for j in 0..SIZE {
            let mut v: Vec<u16> = Vec::new();
            for i in 0..SIZE {
                v.push(World::random_binary());
            }
            cells.push(v);
        }

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
                let cell = self.cells[j][i];
                self.draw_cell(i, j, cell);
            }
        }
    }

    fn count_neighbors(&self, i: usize, j: usize) -> u16 {
        let mut sum: u16 = 0;
        for x in i - 1..i + 2 {
            for y in j - 1..j + 2 {

                if x == i {continue;}
                if y == j {continue;}

                sum = sum + self.get_cell_value(i, j);
            }
        }

        return sum;
    }

    fn get_cell_value(&self, i:usize, j:usize) -> u16 {
        if self.cells.len() -1 > j {
            return 0;
        }

        if self.cells[j].len() - 1 > i {
            return 0;
        }

        return self.cells[j][i];

    }

    fn next_state_cells(&self) {
        let mut copy = self.cells.clone();

        for j in 0..SIZE {
            for i in 0..SIZE {
                let count = self.count_neighbors(i, j);
                let new_state = self.next_state_cell(count); 
            }
        }
    }

    fn next_state_cell(&self, n_count: u16) -> u16 {

        if n_count > 3 {
            return 0;
        }
        
        if n_count == 3 {
            return 1;
        }

         if n_count < 2 {
            return 0;
        }
        return 0;
    }


}

#[macroquad::main("MyGame")]
async fn main() {
    let world = World::new();
    loop {
        clear_background(BLACK);

        world.draw();
        next_frame().await
    }
}
