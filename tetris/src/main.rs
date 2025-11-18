use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("MyGame")]
async fn main() {
    let width = 6.;
    let height = 10.;
    let size = 55.;
    let mut fig1 = vec![];
    let mut fig2 = vec![];
    let mut fig3 = vec![];
    let mut figures = vec![];
    let mut figure = vec![];
    let mut base: Vec<Vec2> = vec![];
    let mut time = 0.;

    fig1.push(vec2(0., 0.));
    fig1.push(vec2(1., 0.));
    fig1.push(vec2(2., 0.));

    fig2.push(vec2(0., 0.));
    fig2.push(vec2(0., 1.));
    fig2.push(vec2(0., 2.));
    fig3.push(vec2(0., 0.));


    figures.push(fig1);
    figures.push(fig2);
    figures.push(fig3);

    figure = figures[RandomRange::gen_range(0, figures.len())].clone();

    loop {
        draw_rectangle_lines(0., 0., size * width, size * (height + 1.), 5., GREEN);

        for p in &figure {
            draw_rectangle(p.x * size, p.y * size, size, size, MAGENTA);
        }

        for p in &base {
            draw_rectangle(p.x * size, p.y * size, size, size, MAGENTA);
        }

        if time > 0.3 {
            time = 0.;

            for p in &mut figure {
                p.y += 1.;
            }
        }

        time += get_frame_time();

        if is_key_pressed(KeyCode::Left) {
            for p in &mut figure {
                p.x += -1.
            }
        }

        if is_key_pressed(KeyCode::Right) {
            for p in &mut figure {
                p.x += 1.
            }
        }

        let mut replace = false;

        if figure.iter().any(|i| i.y >= height) {
            replace = true;
        }

        if !replace {
            for pb in &base {
                for pf in &figure {
                    if pb.x != pf.x {
                        continue;
                    }

                    if pb.y  == pf.y + 1. {
                        replace = true;
                        break;
                    }
                }
            }
        }

        if replace {
            base.append(&mut figure.clone());

            figure = figures[RandomRange::gen_range(0, figures.len())].clone();
        }

        if replace {
            let mut miny: f32 = 1000.;
            let mut maxy: f32 = 0.;

            for p in &base {
                miny = miny.min(p.y);
                maxy = maxy.max(p.y);
            }

            let mut y = 0.;

            while y <= maxy {
                let mut remove = true;

                for x in 0..width as usize {
                    let p = vec2(x as f32, y);

                    if !base.iter()
                        .any(|i| i.x == p.x && i.y == p.y) {
                        remove = false;
                        break;
                    }
                }

                if remove {
                    base.retain(|i| i.y != y);

                    for p in &mut base {
                        if p.y < y {
                            p.y += 1.
                        }
                    }
                }

                y += 1.;
            }
        }

        next_frame().await
    }
}
