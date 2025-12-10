use macroquad::{prelude::*, rand::RandomRange};

const SQUARE_SIZE: f32 = 5.;
const  STEP: f32 = 6.;
struct Agent {
    color: Color,
    path: Vec<Vec2>
}

impl Agent {
    fn new(color: Color) -> Self {
        let mut agent = Agent{
            color: color,
            path: vec![]
        };

        agent.path.push(vec2(screen_width() / 2., screen_height() / 2.));

        return agent
    }

    fn mov(&mut self) {
        let dirs = vec![
            vec2(STEP,0.),
            vec2(-STEP,0.),
            vec2(0.,-STEP),
            vec2(0.,STEP)
        ];

        let index = RandomRange::gen_range(0, dirs.len());

        let dir = dirs[index];

        let n_pos  = dir + self.path[self.path.len() -1];
        self.path.push(n_pos);
    }

    fn draw(&self) {
        for i in 0..self.path.len() -1  {
            let p1 = self.path[i];
            let p2 = self.path[i + 1];

            draw_line(p1.x, p1.y, p2.x, p2.y, 2., self.color);

        }


        let p = self.path[self.path.len() - 1];
        draw_rectangle(p.x, p.y, SQUARE_SIZE, SQUARE_SIZE, self.color);
    }
}

fn random_color() -> Color {
    Color::new(
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        1.0,
    )
}

#[macroquad::main("MyGame")]
async fn main() {

    let mut agents = vec![];

    for i in 0..100 {
        agents.push(Agent::new(random_color()));

    }
    loop {

        for a in &mut agents {
            a.draw();
            a.mov();
        }

        next_frame().await
    }
}
