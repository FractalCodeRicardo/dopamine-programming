use macroquad::{prelude::*, rand::RandomRange};

const  SIZE: f32  =  10.;
const  STEP: f32  =  11.;


struct Agent {
    path: Vec<Vec2>,
    color: Color
}

impl Agent {

    fn new(color: Color) -> Self {

        let mut agent  = Agent {
            path: vec![],
            color: color
        };

        agent.path.push(vec2(screen_width() /2., screen_height()/2.));

        agent
    }


    fn random_dir() -> Vec2 {
        let dirs = vec![
            vec2(STEP,0.),
            vec2(-STEP,0.),
            vec2(0.,STEP),
            vec2(0.,-STEP),
        ];

        let i = RandomRange::gen_range(0, dirs.len());

        return dirs[i];
        
    }

    fn draw(&self)  {
        for i in 0..self.path.len() -1 {
            let p1  = self.path[i];
            let p2  = self.path[i+1];

            draw_line(p1.x, p1.y, p2.x, p2.y, 3., self.color);

        }

        let p = self.path[self.path.len() -1];

        draw_rectangle(p.x, p.y, SIZE, SIZE, self.color);
    }

    fn mov(&mut self) {
        let dir  = Agent::random_dir();
        let p = self.path[self.path.len() -1];


        let n_p  = dir + p;

        self.path.push(n_p);
    }
}
#[macroquad::main("MyGame")]
async fn main() {
    let mut agents = vec![];

   //the simulation is done but lets make it more beauty 
    for i in 0..100 {
        agents.push(Agent::new(Color {
            r: RandomRange::gen_range(0., 1.),
            g: RandomRange::gen_range(0., 1.),
            b: RandomRange::gen_range(0., 1.),
            a:1., 
        }));
    }

    loop {
        clear_background(BLACK);

        for p in &mut agents {
            p.draw();
            p.mov();
        }


        next_frame().await
    }
}
