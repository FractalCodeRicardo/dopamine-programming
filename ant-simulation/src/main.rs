
mod utils;
mod consts;
mod food;
mod pheromone;
mod ant;


use std::collections::HashMap;

use macroquad::{prelude::*, rand::RandomRange};
use consts::*;
use utils::*;

use crate::{ant::{Ant, AntList}, food::FoodList, pheromone::Pheromones};

struct Simulation {
    ant_nest: Vec2,
    ants: AntList,
    food: FoodList,
    pheromones: Pheromones,
}

impl Simulation {
    fn new() -> Self {
        let nest = vec2((WIDTH/2.).floor(), (HEIGHT/2.).floor());
        let  simulation = Simulation {
            ant_nest: nest,
            ants: AntList::new(),
            pheromones: Pheromones::new(),
            food: FoodList::new(),
        };


        return simulation;
    }

    fn draw(&self) {
        self.pheromones.draw();
        self.draw_nest();
        self.food.draw();
        self.ants.draw_ants();
    }


    fn draw_nest(&self) {
        draw_square(
            &self.ant_nest,
            BROWN);
    }

    fn mov(&mut self) {
        self.ants.mov_ants(&self.pheromones);
        self.evaluate_ants_state();
    }


    fn spread_pheronome(&mut self) {
        for a in &self.ants.ants {
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
        for a in &mut self.ants.ants {
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

    fn print(&self) {
        println!("#########");
        self.ants.print();
        self.food.print();
        self.pheromones.print();
        println!("#########");
    }
}

#[macroquad::main("Ants")]
async fn main() {
    let mut simulation = Simulation::new();

    loop {
        clear_background(BLACK);

        // if is_key_pressed(KeyCode::Enter) {
            simulation.mov();
            simulation.spread_pheronome();
            simulation.evaporate_pheromone();


        // }

        simulation.draw();

        next_frame().await
    }
}
