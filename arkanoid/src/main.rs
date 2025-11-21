use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("MyGame")]
async fn main() {
    let width = screen_width() / 10.;
    let height = screen_height() / 10.;
    let mut tiles = vec![];

    for i in 0..10 {
        for j in 0..4 {
            tiles.push(Rect::new(
                i as f32 * width,
                j as f32 * height,
                width - 7.,
                height - 7.,
            ))
        }
    }

    let mut bar = Rect::new(screen_width() / 2. - 100., screen_height() - 70., 200., 30.);

    let mut ball = Rect::new(screen_width() / 2., screen_height() - 300., 30., 30.);

    let mut dir = vec2(5., -10.);

    loop {
        clear_background(DARKPURPLE);

        for t in &tiles {
            draw_rectangle(
                t.x,
                t.y,
                t.w,
                t.h,
                Color {
                    r: RandomRange::gen_range(0., 255.) / 255.,
                    g: RandomRange::gen_range(0., 255.) / 255.,
                    b: RandomRange::gen_range(0., 255.) / 255.,
                    a: 255.,
                },
            );
        }

        draw_rectangle(bar.x, bar.y, bar.w, bar.h, SKYBLUE);
        draw_rectangle(ball.x, ball.y, ball.w, ball.h, GREEN);

        if is_key_down(KeyCode::Left) {
            bar = Rect::new(bar.x -10., bar.y, bar.w, bar.h);
        }

        if is_key_down(KeyCode::Right) {
            bar = Rect::new(bar.x + 10., bar.y, bar.w, bar.h);
        }

        ball = Rect::new(ball.x + dir.x, ball.y + dir.y, ball.w, ball.h);

        for i in 0..tiles.len() {
            let t = &tiles[i];

            let p1 = vec2(ball.x, ball.y - 1.);
            let p2 = vec2(ball.x + ball.w, ball.y - 1.);

            if t.contains(p1) || t.contains(p2) {
                dir *= vec2(1., -1.);
                tiles.remove(i);
                break;
            }
        }

        let bottom = vec2(ball.x + ball.w / 2., 
            ball.y + ball.w + 1.);

        if bar.contains(bottom) {
                
            dir *= vec2(1., -1.);

        }

        if ball.x + ball.w >= screen_width() {
            dir *= vec2(-1., 1.);
        }

        if ball.x <= 0. {
            dir *= vec2(-1., 1.);
        }
        next_frame().await
    }
}
