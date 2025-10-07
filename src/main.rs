use std::arch::x86_64::_mm_andnot_si128;

use macroquad::prelude::*;

fn draw_circles(angle: f32, radius: f32) {
    let x = radius * angle.to_radians().cos();
    let y = radius * angle.to_radians().sin();
    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;
    let mut r = radius;

    while r > 0. {

        draw_circle_lines(x + cx, y + cy, r, 1., WHITE);
        r = r - 5.

    }
}

#[macroquad::main("MyGame")]
async fn main() {

    let mut angle: f32 = 0.;
    let radius: f32 = 200.;

    loop {
        clear_background(BLACK);

        draw_circles(angle, radius);
        draw_circles(angle*-1., radius);

        angle += 1.;

        next_frame().await
    }
}
