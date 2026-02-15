use macroquad::{
    color::{BLACK, WHITE},
    math::{Vec2, vec2},
    prelude::collections,
    shapes::draw_rectangle,
    window::{clear_background, next_frame, screen_height, screen_width},
};

struct Square {
    pos: Vec2,
    size: f32,
}

fn draw(s: &Square) {
    let offset = 100.;
    draw_rectangle(offset + s.pos.x, s.pos.y, s.size, s.size, WHITE);

    draw_rectangle(
        offset + s.pos.x + s.size / 3.,
        s.pos.y + s.size / 3.,
        s.size / 3.,
        s.size / 3.,
        BLACK,
    );
}

fn get_squares(s: Square, deep: i32) -> Vec<Square> {
    let mut squares = vec![];
    if deep ==6  {
        return squares;
    }

    let pos = s.pos;
    let size = s.size;

    squares.push(s);

    for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                continue;
            }

            let pox = pos.x + x as f32 * size /3.;
            let poy = pos.y + y as f32 * size /3.;
            let nsize = size / 3.;


            let np = Square {
                pos: vec2(pox, poy),
                size: nsize
            };

            squares.append(&mut get_squares(np, deep + 1));

        }
    }

    return squares;
}

#[macroquad::main("Sierpinsky")]
async fn main() {
    let parent = Square {
        pos: vec2(0., 0.),
        size: screen_height(),
    };

    let squares = get_squares(parent, 0);

    loop {
        clear_background(WHITE);

        for s in &squares {
            draw(s);
        }

        next_frame().await
    }
}
