
use std::collections::HashMap;

use macroquad::{color::{Color, YELLOW}, math::{vec2, Vec2}};

use crate::{consts::{ANT_RADIUS, FOOD_NUMBER, FOOD_PHEROMONE, FOOD_QUANTITY, FOOD_RADIUS, HEIGHT, HOME_PHEROMONE, MAX_SMELL, WIDTH}, utils::{draw_square, valid_pos}};

pub struct Pheromone {
    pos: Vec2,
    smell: f32,
    p_type: i16
}


pub struct Pheromones {
    food: HashMap<String, Pheromone>,
    home: HashMap<String, Pheromone>,
}

impl Pheromones {
    pub fn new() -> Self {
        Pheromones {
            food: HashMap::new(),
            home: HashMap::new(),
        }
    }

    pub fn get_list(&self) -> Vec<&Pheromone> {
        let food: Vec<&Pheromone> = self
            .food
            .values()
            .collect();

        let home: Vec<&Pheromone> = self
            .home
            .values()
            .collect();

        let mut list = vec![];
        list.extend(food);
        list.extend(home);

        return list;
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

    pub fn evaporate(&mut self) {
        for p in &mut self.food {
               p.1.smell -= 0.01;
               p.1.smell = p.1.smell.max(0.);
        }

        for p in &mut self.home {
               p.1.smell -= 0.01;
               p.1.smell = p.1.smell.max(0.);
        }
    }

    pub fn draw(&self) {
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


    pub fn remove_food(&mut self, pos: &Vec2) {
        let key = Self::get_key(&pos);

        if self.food.contains_key(&key) {
            // println!("Pheromone Food removed {} {}", pos.x, pos.y);
            self.food.remove(&key);
        }

    }

    fn print_pheromone(typ: &str, pheromone: &Pheromone) {
        println!("{}", typ);
        println!("P {}{} S {} T {}", 
            pheromone.pos.x,
            pheromone.pos.y,
            pheromone.smell,
            pheromone.p_type
            )
    }
    
    pub fn add_home_pheromone(&mut self, pos: &Vec2) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: MAX_SMELL,
            p_type: HOME_PHEROMONE
        };

        // Self::print_pheromone("home", &pheromone);
        self.home.insert(key, pheromone);
    }

    pub fn add_food_pheromone(&mut self, pos: &Vec2) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: MAX_SMELL,
            p_type: FOOD_PHEROMONE
        };
        // Self::print_pheromone("food", &pheromone);
        self.food.insert(key, pheromone);
    }

    pub fn add(&mut self, pos: Vec2, p_type: i16) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: 1.,
            p_type: p_type,
        };

        let pheromones = match p_type {
            HOME_PHEROMONE => &mut self.home,
            FOOD_PHEROMONE => &mut self.food,
            _ => panic!("Not supported")
        };

       let exists = pheromones
           .contains_key(&key);

        if !exists {
            // Self::print_pheromone(&p_type.to_string(), &pheromone);
            pheromones.insert(key, pheromone);
            return;
        }

        let c_pheromone: &mut Pheromone = pheromones
            .get_mut(&key)
            .unwrap();

        c_pheromone.smell += 1.;
        c_pheromone.smell.max(MAX_SMELL -1.);
    }



    fn get_key(pos: &Vec2) -> String {
        let x = pos.x as i16;
        let y = pos.y as i16;

        return format!("{}-{}", x, y);
    }

    pub fn print(&self) {
        println!("PHEROMONES ----");
        println!("FOOD:");
        for e in &self.food {
            println!("key {} value {} {} smell {}", e.0, e.1.pos.x, e.1.pos.y, e.1.smell);
        }

        println!("HOME:");
        for e in &self.home {
            println!("key {} value {} {} smell {}", e.0, e.1.pos.x, e.1.pos.y, e.1.smell);
        }
    }
    fn get_pheromone(&self, pos: &Vec2, p_type: i16) -> Option<&Pheromone> {
        let key = Self::get_key(pos);

        if p_type == FOOD_PHEROMONE {
            return self.food.get(&key);
        }

        if p_type == HOME_PHEROMONE {
            return self.home.get(&key);
        }

        return None;
    }

    fn near_pheromone(
        &self,
        pos: &Vec2,
        p_type: i16
    )  
        -> Option<Vec2> {
            
        let neighbors = Self::get_neighbors(pos);
        let mut pheromone: Option<Vec2> = None;
        let mut max = 0.;

        for n in &neighbors {

            let res = self
                .get_pheromone(n, p_type);

            if !res.is_some() {
                continue;
            }

            let phe = res.unwrap();

            if phe.smell > max {
                max = phe.smell;
                pheromone = Some(phe.pos.clone());
            }
        }

        return pheromone;
        
    }

    pub fn near_home_pheromone(
        &self,
        pos: &Vec2
    ) -> Option<Vec2> {
        return self.near_pheromone(pos, HOME_PHEROMONE);
    }

    pub fn near_food_pheromone(
        &self,
        pos: &Vec2
    ) -> Option<Vec2> {
        return self.near_pheromone(pos, FOOD_PHEROMONE);
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
