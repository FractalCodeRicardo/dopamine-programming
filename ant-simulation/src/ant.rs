use macroquad::{color::RED, math::{vec2, Vec2}, rand::RandomRange};

use crate::{consts::{ANTS_NUMBER, HEIGHT, RETURN, SEARCH, WIDTH}, pheromone::Pheromones, utils::{draw_square, valid_pos}};


pub struct Ant {
    pub prev_pos: Vec2,
    pub pos: Vec2,
    dir: Vec2,
    pub state: i16,
}

pub struct AntList {
    pub ants: Vec<Ant>
}

impl AntList {

    pub fn new() -> Self {

        let nest = vec2((WIDTH/2.).floor(), (HEIGHT/2.).floor());
        AntList {
            ants: AntList::get_ants(nest)
        }
    }


    pub fn draw_ants(&self) {
        for a in &self.ants {
            a.draw();
        }
    }

    pub fn mov_ants(&mut self, pheromones: &Pheromones) {
        for a in &mut self.ants {
            a.mov(&pheromones);
        }
    }

    pub fn get_ants(pos: Vec2) -> Vec<Ant> {
        let mut ants = vec![];

        for _i in 0..ANTS_NUMBER {
            ants.push(Ant::new(pos));
        }

        return ants;
    }

    pub fn print(&self) {
        println!("ANTS -------");
        for a in &self.ants {
            a.print();
        }
    }
    
} 

impl Ant {
    pub fn new(pos: Vec2) -> Self {
        Ant {
            prev_pos: pos.clone(),
            pos: pos.clone(),
            state: SEARCH,
            dir: vec2(0., 0.),
        }
    }


    pub fn mov(&mut self, pheromones: &Pheromones) {
        if self.state == SEARCH {
            self.search_food(pheromones);
            return;
        }

        if self.state == RETURN {
            self.return_nest(pheromones);
            return;
        }

        self.explore();
    }

    fn print(&self) {
        let mut status = "Search";
        if self.state == RETURN {
            status = "Return";
        }
        let text = format!("Ant Pos({},{}), {}", 
            self.pos.x,
            self.pos.y,
            status);

        println!("{}", text);

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

    fn search_food(&mut self, pheromones: &Pheromones) {
        let near_pheromone = pheromones
            .near_food_pheromone(&self.pos);

        if near_pheromone.is_none() {
            self.explore();
            return;
        }

        let phe = near_pheromone.unwrap();
        let goto = Self::direction_to(
            &self.pos, 
            &phe
        );

        self.dir = goto;
        let new_pos = self.pos + self.dir;

        if self.prev_pos == new_pos {
            self.explore();
            return;
        }

        self.prev_pos = self.pos;
        self.pos = new_pos;
    }

    fn explore(&mut self) {
        let dir = Ant::random_dir();
        let new_pos = dir + self.pos;

        if valid_pos(&new_pos) {
            self.prev_pos = self.pos;
            self.pos = new_pos;
        }
    }

    fn normalize_step(step: f32) -> f32 {
        if step.abs() == 0. {
            return 0.;
        }

        if step < 0. {
            return -1.;
        }

        return 1.;
    }

    fn direction_to(from: &Vec2, to: &Vec2) -> Vec2 {
        let mut goto = to.clone() - from.clone();
        goto.x = Self::normalize_step(goto.x);
        goto.y = Self::normalize_step(goto.y);

        return goto;
    }

    fn return_nest(&mut self, pheromones: &Pheromones) {
        let near_pheromone = pheromones
            .near_home_pheromone(&self.pos);

        dbg!("Near Home to ", near_pheromone);
        if !near_pheromone.is_some() {
            self.explore();
            return;
        }

        let phe_pos = near_pheromone.unwrap();
        let goto = Self::direction_to(&self.pos, &phe_pos);

        self.dir = goto;

        let new_pos = self.dir + self.pos;

        if self.prev_pos == new_pos {
            self.explore();
            return;
        }

        self.prev_pos = self.pos;
        self.pos = new_pos;
    }

    pub fn draw(&self) {
        draw_square(&self.pos, RED);
    }
}
