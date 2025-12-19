use std::{thread::sleep, time::Duration};

use rand::Rng;

const WIDTH: f32 = 90.;
const HEIGHT: f32= 25.;

struct P {
    x: f32,
    y:f32
}

struct Rect {
    pos: P,
    dir: P,
    color: i16
}

fn rect_points(p: &P) -> Vec<P>{
    let mut points = vec![];
    for y in 0..4 {
        for x in 0..6 {
            points.push(P {
                x: p.x + x as f32,
                y: p.y + y as f32
            });
        }
    }

    points
}

fn draw_rects(rects: &Vec<Rect>) {
    let w = WIDTH as usize;
    let h = HEIGHT as usize;

    let mut screen = vec![vec![String::from(" "); w]; h];

    for rect in rects {

        let points = rect_points(&rect.pos);

        for p in points {

            if p.x < 0. || p.x >= WIDTH {
                continue;
            }

            if p.y < 0. || p.y >= HEIGHT {
                continue;
            }

            let x = p.x as usize;
            let y = p.y as usize;

            screen[y][x] = format!(
                "\x1b[{}m{}\x1b[0m", rect.color, "@"
                )
        }
    }

    for line in screen {
        let mut text = String::new();

        for l in line {
            text = format!("{}{}", text, l);
        }

        println!("{text}");
    }
}


fn main() {
    let mut rects = vec![];
    let mut rd = rand::rng();
    let ones = vec![1.,-1.];
    for i in 0..10 {
        let pos = P{
            x: rd.random_range(0. .. WIDTH -1.),
            y: rd.random_range(0. .. HEIGHT -1.),
        };

        let dir = P {
            x: ones[rd.random_range(0 .. 2)],
            y: ones[rd.random_range(0 .. 2)],
        };
        
        let color: i16 = rd.random_range(31 .. 38);

        let rect = Rect {
            pos,
            dir,
            color
        };

        rects.push(rect);
    }
    
    loop {
        println!("\x1b[2J\x1b[H");

        for r in &mut rects {
            let pos = &mut r.pos;
            let dir = &mut r.dir;

            pos.x += dir.x;
            pos.y += dir.y;

            if pos.x <=0. || pos.x >= WIDTH {
                dir.x *= -1.;
            }

            if pos.y <=0. || pos.y >= HEIGHT {
                dir.y *= -1.;
            }

        }

        draw_rects(&rects);

        sleep(Duration::from_millis(50));
    }
}
