use macroquad::{color::{BLACK, GREEN}, shapes::{draw_circle, draw_line}, window::{clear_background, next_frame, screen_height, screen_width}};

#[macroquad::main("Pendulum")]
async fn main() {
    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;
    let L = 250.;
    let g = 9.8;
    let mut angle: f32= 1.0;
    let mut v = 0.;
    let dt = 0.1;


    loop {

        clear_background(BLACK);

        // the formula is
        // a'' = - g/L * sin angle

        // calculare acceleration
        let aa = - (g/L) * angle.sin();

        // integrate acceleration to get velocity

        v += aa * dt;

        // integrate to get new  angle
        
        angle += v * dt;

        // get x and y from angle
        
        let x = L * angle.sin() + cx;
        let y = L * angle.cos() + cy;

        draw_line(cx, cy, x, y , 10.,GREEN);
        draw_circle(x, y, 50.,GREEN);

        next_frame().await;

    }
}
