use macroquad::{prelude::*, rand::RandomRange, telemetry::frame};

struct Vec(i16, i16);
struct Ant(Vec, i16, Color);
#[derive(Clone)]
struct Square(bool, Color);

#[macroquad::main("Ants")]
async fn main() {
    let height = 100;
    let width = 100;
    let size = 10;
    let mut ants = vec![];
    let mut squares = vec![vec![Square(false, BLACK); width]; height];

    let angles = vec![0, 90, 180, 270];

    for i in 0..100 {
        let x = RandomRange::gen_range(0, width) as i16;
        let y = RandomRange::gen_range(0, height) as i16;
        let pos = Vec(x, y);
        let angle = angles[RandomRange::gen_range(0, angles.len())];
        let color = Color {
            r: RandomRange::gen_range(0., 1.),
            g: RandomRange::gen_range(0., 1.),
            b: RandomRange::gen_range(0., 1.),
            a: 1.
        };

        let ant = Ant(pos, angle, color);

        ants.push(ant);
    }

    loop {
        clear_background(BLACK);

        for j in 0..height {
            for i in 0..width {
                let s = &squares[j][i];

                if s.0  {
                    draw_rectangle(
                        (i * size) as f32,
                        (j * size) as f32,
                         size as f32, size as f32, s.1);
                }
            }
        }

        for a in &mut ants {
            let pos = &mut a.0;

            if pos.0 < 0 || pos.0 >= width as i16 {
                continue;
            }

            if pos.1 < 0 || pos.1 >= height as i16 {
                continue;
            }

            let s = &mut squares[pos.1 as usize][pos.0 as usize];

            if s.0 == false {
                s.0 = true;
                s.1 = a.2;
                a.1 += 90;
            } else {
                s.0 = false;
                s.1 = BLACK;
                a.1 += -90;
            }

            let radians = (a.1 as f32).to_radians();
            let x = radians.cos();
            let y = radians.sin();

            pos.0 += x as i16;
            pos.1 += y as i16;

        }
        next_frame().await;
    }
}

