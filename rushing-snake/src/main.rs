use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("MyGame")]
async fn main() {
    let delay: f64 = 0.3;
    let mut time = get_time();
    let mut food = vec2(7., 7.);
    let mut dir = vec2(1., 0.);
    let mut snake: Vec<Vec2> = vec![];

    snake.push(vec2(5., 5.));

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Left) {
            dir = vec2(-1., 0.);
        }

        if is_key_pressed(KeyCode::Right) {
            dir = vec2(1., 0.);
        }

        if is_key_pressed(KeyCode::Up) {
            dir = vec2(0., -1.);
        }

        if is_key_pressed(KeyCode::Down) {
            dir = vec2(0., 1.);
        }

        for s in &snake {
            draw_rectangle(s.x * 30., s.y * 30., 30., 30., GREEN);
        }

        draw_rectangle(food.x * 30., food.y * 30., 30., 30., MAGENTA);

        if get_time() - time > delay {
            time = get_time();


            if snake[0] + dir == food {
                snake.insert(0, snake[0] + dir);
                food = vec2(
                    RandomRange::gen_range(0, 15) as f32,
                    RandomRange::gen_range(0, 15) as f32,
                )
            } else {
                let mut i = snake.len() -1;

                while i > 0 {
                    snake[i] = snake[i-1];
                    i -= 1;
                }

                snake[0] = snake[0] + dir;
            }
        }
        next_frame().await
    }
}
