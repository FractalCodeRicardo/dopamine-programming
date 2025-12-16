use macroquad::{color::Color, math::Vec2, miniquad::gl::WGL_DEPTH_BITS_ARB, shapes::draw_rectangle, window::screen_width};

use crate::consts::{ WIDTH};

pub fn draw_square(pos: &Vec2, color: Color) {

    let mut size: f32 = screen_width() / WIDTH;
    println!("SIZE {}", size);
    size = size.floor();
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
