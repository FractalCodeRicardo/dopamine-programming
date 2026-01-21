use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("Planets")]
async fn main() {
    let mut planets = vec![];


    for i in 0..100 {
        planets.push(Planet::new(i as f32 * 1.))
    }

    loop {
        clear_background(BLACK);

        for p in &mut planets {
            p.draw();
        }

        next_frame().await;
    }
}

struct Planet {
    size: f32,
    radius: f32,
    angle: f32,
    color: Color,
    a: f32,
    b: f32,
    history: Vec<Vec2>,
}

impl Planet {
    fn new( radius: f32,) -> Self {
        Planet {
            size: RandomRange::gen_range(5., 20.),
            radius: radius,
            angle: RandomRange::gen_range(1., 360.),
            color: Color {
                r: RandomRange::gen_range(0., 1.),
                g: RandomRange::gen_range(0., 1.),
                b: RandomRange::gen_range(0., 1.),
                a: 1.

            },
            a: RandomRange::gen_range(0.5, 1.),
            b: RandomRange::gen_range(0.5, 1.),
            history: vec![],
        }
    }

    fn draw(&mut self) {
        let cx = screen_width() / 2.;
        let cy = screen_height() / 2.;

        let px = self.a * self.radius * self.angle.to_radians().cos();

        let py = self.b * self.radius * self.angle.to_radians().sin();

        draw_circle(cx + px, cy + py, self.size / 2., self.color);

        self.history.push(vec2(cx + px, cy + py));

        // if self.history.len() > 3 {
        //     for i in 4..self.history.len() - 1 {
        //
        //         if i >= 500 {
        //             break;
        //         }
        //         let p1 = self.history[i];
        //         let p2 = self.history[i + 1];
        //
        //         draw_line(p1.x, p1.y, p2.x, p2.y, 1., self.color);
        //     }
        // }

        self.angle += 5.
    }
}
