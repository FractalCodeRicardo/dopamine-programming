use macroquad::{prelude::*, rand::RandomRange};

struct Point {
    x: f32,
    y: f32,
}

fn get_triangle_points() -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;
    let radius = 250.;

    points.push(Point {
        x: cx,
        y: cy - radius,
    });

    points.push(Point {
        x: cx - radius,
        y: cy + radius,
    });

    points.push(Point {
        x: cx + radius,
        y: cy + radius,
    });

    return points;
}

fn get_points() -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let triangle_points = get_triangle_points();
    let mut point = Point {
        x: triangle_points[0].x + 40.,
        y: triangle_points[0].y + 150.,
    };

    for tp in &triangle_points {
        points.push(Point { x: tp.x, y: tp.y });
    }

    let mut i_triangle = 0;
    while points.len() < 10000 {
        points.push(Point {
            x: point.x,
            y: point.y,
        });

        point = get_next_point(&triangle_points[i_triangle], point);
        i_triangle = RandomRange::gen_range(0, triangle_points.len());
    }

    return points;
}

fn get_next_point(t_point: &Point, point: Point) -> Point {
    return Point {
        x: (t_point.x + point.x) / 2.,
        y: (t_point.y + point.y) / 2.,
    };
}

fn draw_point(point: &Point) {
    draw_circle(point.x, point.y, 2., GREEN);
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut i = 0;
    let points = get_points();
    loop {
        clear_background(BLACK);

        for j in 0..i {
            draw_point(&points[j]);
        }

        i += 100;

        if i > points.len() {
            i = 0;
        }

        next_frame().await
    }
}
