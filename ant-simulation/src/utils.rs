use macroquad::{color::Color, math::Vec2, shapes::draw_rectangle};

use crate::consts::{SQUARE_SIZE, WIDTH};

pub fn draw_square(pos: &Vec2, color: Color) {
    let size: f32 = SQUARE_SIZE;
    draw_rectangle(
        pos.x * size,
        pos.y *size,
        size,
        size,
        color
    );

}


pub fn valid_pos(pos: &Vec2) -> bool {

    if pos.x < 0. || pos.x >WIDTH {
        return false;
    }

    if pos.y < 0. || pos.y >WIDTH {
        return false;
    }

    return true;
}
