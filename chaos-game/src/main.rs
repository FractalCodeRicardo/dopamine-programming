use macroquad::{prelude::*, rand::RandomRange};

const SIDES: f32 = 5.;
const RADIUS: f32 = 350.;
const ITERATIONS: usize = 100000;

struct Point {
    x: f32,
    y: f32,
}

fn get_figure_points() -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let degrees: f32 = 360. / SIDES;
    let mut c_degree: f32 = 0.;
    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;

    while c_degree <= 360. {
        let x = RADIUS * c_degree.to_radians().cos();
        let y = RADIUS * c_degree.to_radians().sin();

        points.push(Point {
            x: cx + x,
            y: cy + y,
        });
        c_degree += degrees;
    }

    return points;
}

fn get_points() -> Vec<Point> {
    let figure_points = get_figure_points();
    let mut points: Vec<Point> = Vec::new();
    let mut point = Point {
        x: screen_width() / 2.,
        y: screen_height() / 2.,
    };

    for fp in &figure_points {
        points.push(Point { x: fp.x, y: fp.y })
    }

    let mut i = 0;
    let mut i_figure = 0;

    while i <= ITERATIONS {
        points.push(Point {
            x: point.x,
            y: point.y,
        });

        let fp = &figure_points[i_figure];

        point = Point {
            x: (fp.x + point.x) / 2.,
            y: (fp.y + point.y) / 2.,
        };

        i_figure = RandomRange::gen_range(0, figure_points.len());

        i += 1;
    }

    return points;
}

fn draw_points(points: &Vec<Point>, until: usize) {
    for i in 0..until {
        let p = &points[i];
        draw_circle(p.x, p.y, 1., GREEN)
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let points = get_points();
    let mut i = 0;
    loop {
        clear_background(BLACK);

        draw_points(&points, i);

        i += 800;

        if i > points.len() {
            i = 0;
        }

        next_frame().await
    }
}
