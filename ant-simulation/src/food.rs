

use macroquad::{color::YELLOW, math::{vec2, Vec2}};

use crate::{consts::{FOOD_NUMBER, FOOD_QUANTITY, FOOD_RADIUS, HEIGHT, WIDTH}, utils::draw_square};


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
        let step = 360. / FOOD_NUMBER as f32;

        while angle < 360. {
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
        for i in 0..self.food.len() {
            let p = &self.food[i];
            if &p.pos == pos {
                return Some(i);
            }
        }

        return None;
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

    pub fn print(&self) {

        println!("Food ---");
        for f in &self.food {
            println!("Food {}{} Q: {}", f.pos.x, f.pos.y, f.quantity);
        }

    }
}

