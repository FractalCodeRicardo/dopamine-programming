use macroquad::{
    color::{Color, BLACK, BLUE, BROWN, DARKBLUE, DARKGRAY, DARKGREEN, GREEN, MAGENTA, PINK, PURPLE, RED, YELLOW}, math::{vec2, Vec2}, rand::RandomRange, shapes::{draw_circle, draw_circle_lines}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width}
};

struct Ball {
    vel: Vec2,
    pos: Vec2,
    color: Color
}

impl Ball {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 10., self.color);
    }

    fn mov(&mut self) {
        let gravity = 9.8;
        let dt = get_frame_time();
        let degradation = 0.9;

        let vx = self.vel.x;
        let vy = self.vel.y + (dt * gravity);

        let mut nvel = vec2(vx, vy);
        let npos = self.pos + nvel;

        if npos.y >= screen_height() {
            nvel.y *= -1.;
            nvel *= degradation;
        }

        if npos.x >= screen_width() || npos.x <= 0. {
            nvel.x *= -1.;
            nvel *= degradation;
        }

        self.vel = nvel;
        self.pos += self.vel;
    }
}


#[macroquad::main("Main")]
async fn main() {
    let mut balls = vec![];
    let colors = vec![
        GREEN,
        YELLOW,
        PINK,
        PURPLE,
        RED,
        BLUE,
        MAGENTA,
        BROWN,
        DARKBLUE,
        DARKGRAY,
        DARKGREEN
    ];

    for _ in 0..200 {
        let pos = vec2(
            RandomRange::gen_range(0., screen_width()),
            RandomRange::gen_range(0., screen_height())
        );

        let vel = vec2(
                RandomRange::gen_range(0., 1.) * 15.,
                RandomRange::gen_range(0., 1.) * 10.,
        );

        let color = colors[RandomRange::gen_range(0, colors.len())];
        let mut ball = Ball { pos: pos, vel: vel, color: color };

        balls.push(ball);
    }

    loop {
        clear_background(BLACK);

        for b in &mut balls {
            b.draw();
            b.mov();
        }
        next_frame().await;
    }
}
