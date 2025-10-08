use macroquad::prelude::*;

fn draw_circles(angle: f32, radius: f32, color: Color) {
    let x = radius * angle.to_radians().cos();
    let y = radius * angle.to_radians().sin();

    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;

    let mut r = radius;

    while r > 0. {
        draw_circle_lines(x + cx, y + cy, r, 2., color);
        r = r - 5.;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let radius: f32 = 200.;
    let mut angle: f32 = 0.;

    loop {
        clear_background(BLACK);

        draw_circles(angle, radius, GREEN);
        draw_circles(angle+45., radius, YELLOW);
        draw_circles((angle+45.) * -1., radius, PINK);
        draw_circles(angle * -1., radius, BLUE);

        angle = angle + 1.;

        next_frame().await
    }
}
