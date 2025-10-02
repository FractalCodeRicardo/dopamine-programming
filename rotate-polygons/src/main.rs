use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut rotation = 0.;
    let mut size = screen_width() / 2.;
    let sides = 10;
    let cx = screen_width() / 2.;
    let cy = screen_height() / 2. + 200.;
    let mut data: Vec<(f32, f32)> = Vec::new();
    clear_background(BLACK);
    loop {

        data.push((size, rotation));

        for d in &data {
            draw_poly_lines(cx, cy, sides, d.0, d.1, 2., GREEN)
        }

        size = size - 5.;
        rotation = rotation + 2.;

        next_frame().await
    }
}
