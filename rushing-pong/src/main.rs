use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut ball = vec2(30., 30.);
    let mut dir = vec2(15., 15.);
    let mut pad = vec2(10., 10.);
    let size = 40.;

    loop {
        clear_background(BLACK);

        draw_rectangle(ball.x, ball.y, size, size, MAGENTA);
        draw_rectangle(pad.x, pad.y, size, size*6., MAGENTA);

        if is_key_down(KeyCode::Up) {
            pad += vec2(0., -10.);
        }


        if is_key_down(KeyCode::Down) {
            pad += vec2(0., 10.);
        }

        if Rect::new(ball.x, ball.y, size, size).overlaps(
            &Rect::new(pad.x, pad.y, size, size*6.)
        ){
            dir = vec2(dir.x * -1., dir.y);
        }

        ball += dir;


        if dir.x > 0. && ball.x + size >= screen_width() {
            dir = vec2(dir.x * -1., dir.y);
        }

        if dir.x < 0. && ball.x <= 0. {
            dir = vec2(dir.x * -1., dir.y);
        }

        if dir.y > 0. && ball.y + size >= screen_height() {
            dir = vec2(dir.x , dir.y *-1.);
        }

        if dir.y < 0. && ball.y <= 0. {
            dir = vec2(dir.x, dir.y * -1.);
        }

        next_frame().await
    }
}
