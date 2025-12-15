use std::collections::HashMap;

use macroquad::{prelude::*, rand::RandomRange};

const HEIGHT: f32 = 3.;
const WIDTH: f32 = 3.;

const SEARCH: i16 = 1;
const RETURN: i16 = 2;

const ANT_RADIUS: f32 = 5.;

const FOOD_PHEROMONE: i16 = 1;
const HOME_PHEROMONE: i16 = 2;


fn draw_square(pos: &Vec2, color: Color) {
    let size: f32 = 50.;
    draw_rectangle(
        pos.x * size,
        pos.y *size,
        size,
        size,
        color
    );

}


    fn valid_pos(pos: &Vec2) -> bool {

        if pos.x < 0. || pos.x >WIDTH {
            return false;
        }

        if pos.y < 0. || pos.y >WIDTH {
            return false;
        }

        return true;
    }

struct Pheromone {
    pos: Vec2,
    smell: f32,
    p_type: i16,
}


struct Pheromones {
    pheromones: HashMap<String, Pheromone>,
}

impl Pheromones {
    fn new() -> Self {
        Pheromones {
            pheromones: HashMap::new(),
        }
    }

    fn get_list(&self) -> Vec<&Pheromone> {
        return self
            .pheromones
            .values()
            .collect();
    }

    fn get_food_phe_color(intensity: f32) -> Color {
        let color = intensity / 3.;
        return Color {
            r: 0.,
            g: 1.,
            b: 0.,
            a: color

        }
    }

    fn get_home_phe_color(intensity: f32) -> Color {
        let color  =  intensity / 3.;
        return Color {
            r: 0.,
            g: 0.,
            b: 1.,
            a: color
        }
    }

    fn evaporate(&mut self) {
        for p in &mut self.pheromones {
               p.1.smell -= 0.01;
               p.1.smell = p.1.smell.max(0.);
        }
    }

    fn draw(&self) {
        for p in self.get_list() {
            let pos = p.pos;

            if p.p_type == FOOD_PHEROMONE {
                let color = Self::get_food_phe_color(p.smell);
                draw_square(&pos, color);
            }

            if p.p_type == HOME_PHEROMONE {
                let color = Self::get_home_phe_color(p.smell);
                draw_square(&pos, color);
            }
        }
    }


    fn add(&mut self, pos: Vec2, p_type: i16) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: 1.,
            p_type: p_type
        };

       let exists = self.pheromones
           .contains_key(&key);


        if !exists {
            self.pheromones.insert(key, pheromone);
            return;
        }

        let c_pheromone: &mut Pheromone = self
            .pheromones
            .get_mut(&key)
            .unwrap();

        let same_type = c_pheromone.p_type == pheromone.p_type;

        if same_type {

            c_pheromone.smell += 1.;
            return;

        } 

        c_pheromone.smell -= 1.;

        if c_pheromone.smell <= 0. {
            self.pheromones.insert(key, pheromone);
            return;
        }
    }


    fn get_key(pos: &Vec2) -> String {
        let x = pos.x as i16;
        let y = pos.y as i16;

        return format!("{}-{}", x, y);
    }

    fn get_pheromone(&self, pos: &Vec2) -> Option<&Pheromone> {
        let key = Self::get_key(pos);

        let pheromone = self.pheromones.get(&key);

        return pheromone;
    }

    fn near_pheromone(
        &self,
        pos: &Vec2,
        p_type: i16
    )  
        -> Option<Vec2> {
            
        let neighbors = Self::get_neighbors(pos);
        let mut pheromone: Option<Vec2> = None;
        let max = 0.;

        for n in &neighbors {

            let res = self
                .get_pheromone(n);

            if !res.is_some() {
                continue;
            }

            let phe = res.unwrap();

            if phe.p_type != p_type {
                continue;
            }

            if phe.smell > max {
                pheromone = Some(phe.pos.clone());
            }
        }

        return pheromone;
        
    }

    fn near_home_pheromone(
        &self,
        pos: &Vec2
    ) -> Option<Vec2> {
        return self.near_pheromone(pos, HOME_PHEROMONE);
    }


    fn get_neighbors(pos: &Vec2) -> Vec<Vec2> {
        let mut neighbors = vec![];
        let positions: Vec<Vec2> = vec![
            vec2(-1. ,-1.),
            vec2(-1. ,0.),
            vec2(-1. ,1.),
            vec2(1. ,-1.),
            vec2(1. ,0.),
            vec2(1. ,1.),
            vec2(0. ,-1.),
            vec2(0. ,1.),
        ];

       let mut radius = 1.;
       while radius <= ANT_RADIUS {
           for e in &positions {
               let n = e.clone() * radius;
               let n_pos = n + pos.clone();

               if valid_pos(&n_pos) {
                    neighbors.push(n_pos);
               }
           }

           radius += 1.;
       }

        return neighbors;
    }

}

struct Ant {
    pos: Vec2,
    dir: Vec2,
    state: i16,
}

impl Ant {
    fn new(pos: Vec2) -> Self {
        Ant {
            pos: pos.clone(),
            state: SEARCH,
            dir: vec2(0., 0.),
        }
    }


    fn mov(&mut self, pheromones: &Pheromones) {
        if self.state == SEARCH {
            self.explore();
            return;
        }

        if self.state == RETURN {
            self.return_nest(pheromones);
            return;
        }

        self.explore();
    }

    fn random_dir() -> Vec2 {
        let dirs = vec![
            vec2(1., 0.),
            vec2(-1., 0.),
            vec2(0., 1.),
            vec2(0., -1.),
        ];

        let index = RandomRange::gen_range(0, dirs.len());

        return dirs[index];
    }

    fn explore(&mut self) {
        let dir = Ant::random_dir();
        let new_pos = dir + self.pos;

        if valid_pos(&new_pos) {
            self.pos = new_pos;
        }
    }

    fn return_nest(&mut self, pheromones: &Pheromones) {
        let near_pheromone = pheromones
            .near_home_pheromone(&self.pos);

        if !near_pheromone.is_some() {
            self.explore();
            return;
        }

        let phe_pos = near_pheromone.unwrap();
        let goto = phe_pos - self.pos;
        let goto_magnitude = goto.length();
        let scale = 1. / goto_magnitude;

        self.dir = goto * scale;

        let new_pos = self.dir + self.pos;

        self.pos = new_pos;
    }

    fn draw(&self) {
        draw_square(&self.pos, RED);
    }
}

struct Simulation {
    ant_nest: Vec2,
    ants: Vec<Ant>,
    food: Vec<Vec2>,
    pheromones: Pheromones,
}

impl Simulation {
    fn new() -> Self {
        let nest = vec2(WIDTH/2., HEIGHT/2.);
        Simulation {
            ant_nest: nest,
            ants: Simulation::get_ants(nest),
            pheromones: Pheromones::new(),
            food: Simulation::get_food(),
        }
    }

    fn draw(&self) {
        self.pheromones.draw();
        self.draw_ants();
        self.draw_nest();
        self.draw_food();
    }

    fn draw_ants(&self) {
        for a in &self.ants {
            a.draw();
        }
    }

    fn draw_food(&self) {
        for f in &self.food {
            draw_square(f, YELLOW);
        }
    }

    fn draw_nest(&self) {
        draw_square(
            &self.ant_nest,
            BROWN);
    }

    fn mov(&mut self) {
        self.mov_ants();
        self.evaluate_ants_state();
    }

    fn mov_ants(&mut self) {
        for a in &mut self.ants {
            a.mov(&self.pheromones);
        }
    }

    fn get_ants(pos: Vec2) -> Vec<Ant> {
        let mut ants = vec![];

        ants.push(Ant::new(pos));
        return ants;
    }

    fn get_food() -> Vec<Vec2> {
        let mut food = vec![];

        food.push(vec2(WIDTH, HEIGHT));

        return food;
    }

    fn spread_pheronome(&mut self) {
        for a in &self.ants {
            let pos = a.pos;

            if a.state == SEARCH {
                self.pheromones.add(pos, HOME_PHEROMONE);
            }

            if a.state == RETURN {
                self.pheromones.add(pos, FOOD_PHEROMONE);
            }
        }
    }

    fn evaporate_pheromone(&mut self) {
        self.pheromones.evaporate();
    }

    fn evaluate_ants_state(&mut self) {
        let food = &self.food;
        for a in &mut self.ants {
            if a.state == SEARCH {
                let found_food = Self::found_food(food, a);

                if found_food {
                    a.state = RETURN;
                    continue;
                }
            }

            if a.state == RETURN {
                let in_home = Self::is_in_home(&self.ant_nest, a);

                if in_home {
                    a.state = SEARCH;
                    continue;
                }
            }
        }
    }

    fn found_food(food: &Vec<Vec2>, ant: &Ant) -> bool {
        food.iter().any(|i| *i == ant.pos)
    }

    fn is_in_home(home: &Vec2, ant: &Ant) -> bool {
        ant.pos == *home
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut simulation = Simulation::new();

    loop {
        clear_background(BLACK);

        simulation.draw();

        if is_key_pressed(KeyCode::Enter) {
            simulation.spread_pheronome();
            simulation.mov();
            simulation.evaporate_pheromone();
        }

        next_frame().await
    }
}
