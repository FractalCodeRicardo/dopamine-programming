
mod utils;
mod consts;
mod enviroment;
mod ant;

use std::collections::HashMap;

use macroquad::{prelude::*, rand::RandomRange};
use consts::*;
use utils::*;

use crate::{ant::Ant, enviroment::{Food, FoodList, Pheromones}};

struct Simulation {
    ant_nest: Vec2,
    ants: Vec<Ant>,
    food: FoodList,
    pheromones: Pheromones,
}

impl Simulation {
    fn new() -> Self {
        let nest = vec2((WIDTH/2.).floor(), (HEIGHT/2.).floor());
        let  simulation = Simulation {
            ant_nest: nest,
            ants: Simulation::get_ants(nest),
            pheromones: Pheromones::new(),
            food: FoodList::new(),
        };


        return simulation;
    }

    fn draw(&self) {
        self.pheromones.draw();
        self.draw_nest();
        self.food.draw();
        self.draw_ants();
    }

    fn draw_ants(&self) {
        for a in &self.ants {
            a.draw();
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

        for i in 0..ANTS_NUMBER {
            ants.push(Ant::new(pos));
        }

        return ants;
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

        self.pheromones.add_home_pheromone(&self.ant_nest);
        for f in &self.food.food {
            self.pheromones.add_food_pheromone(&f.pos);
        }
    }

    fn evaporate_pheromone(&mut self) {
        self.pheromones.evaporate();
    }

    fn evaluate_ants_state(&mut self) {
        let food_list = &mut self.food;
        for a in &mut self.ants {
            if a.state == SEARCH {
                let res = food_list
                    .take(&a.pos);

                if res.removed {
                    self.pheromones.remove_food(&a.pos);
                }
                
                if res.taked {
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

        // if is_key_pressed(KeyCode::Enter) {
            simulation.spread_pheronome();
            simulation.mov();
            simulation.evaporate_pheromone();
        // }

        next_frame().await
    }
}
