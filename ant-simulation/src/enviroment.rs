use std::{cell::RefCell, collections::HashMap};

use macroquad::{color::{Color, YELLOW}, math::{vec2, Vec2}};

use crate::{consts::{ANT_RADIUS, FOOD_NUMBER, FOOD_PHEROMONE, FOOD_QUANTITY, FOOD_RADIUS, HEIGHT, HOME_PHEROMONE, WIDTH}, utils::{draw_square, valid_pos}};

pub struct TakeResult {
    pub taked: bool,
    pub removed: bool
}

impl TakeResult {
    pub fn new(taked: bool, removed: bool) ->Self {
        TakeResult {
            taked: taked,
            removed:removed 
        }
    }
}

pub struct Food {
    pub quantity: f32,
    pub pos: Vec2
}

impl Food {
    pub fn new(pos: Vec2) -> Self{
        Food {
            quantity: FOOD_QUANTITY,
            pos: pos
        }
    }

    pub fn take(&mut self) {
        if self.quantity> 0. {
            self.quantity -= 1.;
        }
    }
}

pub struct FoodList {
    pub food: Vec<Food>
}


impl FoodList {

    pub fn new() -> Self {
        FoodList {
            food: Self::get_food()
        }
    }

    fn get_food() -> Vec<Food> {
        let mut food = vec![];

        // let cx = WIDTH / 2.;
        // let cy = HEIGHT /2.;
        // food.push(Food::new(vec2(cx + 2., cy + 2.)));
       
        let mut angle: f32 = 0.;
        let step = 365. / FOOD_NUMBER as f32;

        while angle <= 365. {
            let mut x = FOOD_RADIUS * angle.to_radians().cos();
            let mut y = FOOD_RADIUS * angle.to_radians().sin();

            x = x + WIDTH / 2.;
            y = y + HEIGHT / 2.;

            x = x.floor();
            y = y.floor();

            let pos = vec2(x, y);
            food.push(Food::new(pos));
            angle += step;
        }

        return food;
    }

    pub fn find_index(&mut self, pos:&Vec2) -> Option<usize>  {
        let res = self
            .food
            .iter().position(|i| i.pos == pos.clone());

        return res;
    }

    pub fn take(&mut self, pos: &Vec2) -> TakeResult {
        
        let res = self.find_index(pos);

        if res.is_none() {
            return TakeResult::new(false, false);
        }

        let index = res.unwrap();
        let food = &mut self.food[index];
        food.take();

        let remove  =food.quantity == 0. ;
        if remove {
            self.food.remove(index);
        }

        return TakeResult::new(true, remove);
    }

    pub fn draw(&self) {
        for f in &self.food {
            draw_square(&f.pos, YELLOW);
        }
    }
}

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
            self.food.remove(&key);
        }

    }
    
    pub fn add_home_pheromone(&mut self, pos: &Vec2) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: 255.,
            p_type: HOME_PHEROMONE
        };
        self.home.insert(key, pheromone);
    }

    pub fn add_food_pheromone(&mut self, pos: &Vec2) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: 255.,
            p_type: FOOD_PHEROMONE
        };
        self.home.insert(key, pheromone);
    }

    pub fn add(&mut self, pos: Vec2, p_type: i16) {
        let key = Self::get_key(&pos);
        let pheromone = Pheromone {
            pos: vec2(pos.x, pos.y),
            smell: 1.,
            p_type: p_type,
        };

        let pheromones = if p_type == HOME_PHEROMONE {
            &mut self.home
        } else {
            &mut self.food
        };

       let exists = pheromones
           .contains_key(&key);

        if !exists {
            pheromones.insert(key, pheromone);
            return;
        }

        let c_pheromone: &mut Pheromone = pheromones
            .get_mut(&key)
            .unwrap();

        c_pheromone.smell += 1.;
    }


    fn get_key(pos: &Vec2) -> String {
        let x = pos.x as i16;
        let y = pos.y as i16;

        return format!("{}-{}", x, y);
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
        let max = 0.;

        for n in &neighbors {

            let res = self
                .get_pheromone(n, p_type);

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
