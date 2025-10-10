use macroquad::prelude::*;

fn draw_polygons(degree: f32, radius: f32, color: Color) {
    let x = radius * degree.to_radians().cos();
    let y = radius * degree.to_radians().sin();

    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;
    let mut r = radius * 0.8;

    while r > 0. {
        draw_poly_lines(x + cx, y + cy, 5, r, degree, 1., color);
        r = r - 20.;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut degree: f32 = 0.;
    let radius: f32 = 200.;

    loop {
        clear_background(BLACK);

        draw_polygons(degree, radius, RED);
        draw_polygons(degree + 45., radius, GREEN);
        draw_polygons(degree + 90., radius, BLUE);
        draw_polygons(degree + 135., radius, PURPLE);

        draw_polygons(degree * -1., radius, YELLOW);
        draw_polygons((degree + 45.) * -1., radius, MAGENTA);
        draw_polygons((degree + 90.) * -1., radius, ORANGE);
        draw_polygons((degree + 135.) * -1., radius, WHITE);

        degree += 2.;
        next_frame().await
    }
}
