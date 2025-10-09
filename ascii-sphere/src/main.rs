use macroquad::prelude::*;

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

struct Point2D {
    x: f32,
    y: f32,
}

const RADIUS: f32 = 100.;
const CENTER: Point3D = Point3D {
    x: 100.,
    y: 100.,
    z: 100.,
};

fn get_sphere_points() -> Vec<Point3D> {
    let mut points: Vec<Point3D> = Vec::new();
    let mut theta: f32 = 0.;
    let mut phi: f32 = 0.;

    while theta <= 360. {
        phi = 0.;
        while phi <= 360. {
            let p = get_point(theta.to_radians(), phi.to_radians());

            points.push(p);

            phi += 3.
        }
        theta += 3.
    }

    return points;
}

fn rotate_x(points: &Vec<Point3D>, angle: f32) -> Vec<Point3D> {
    let theta = angle.to_radians();
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let mut new_points: Vec<Point3D> = Vec::new();

    for p in points {
        new_points.push(Point3D {
            x: p.x,
            y: p.y * cos_theta - p.z * sin_theta,
            z: p.z * sin_theta + p.z * cos_theta,
        });
    }

    return new_points;
}
fn get_point(theta: f32, phi: f32) -> Point3D {
    let x = RADIUS * theta.sin() * phi.cos();
    let y = RADIUS * theta.sin() * phi.sin();
    let z = RADIUS * theta.cos();

    return Point3D { x: x, y: y, z: z };
}

fn project(points: &Vec<Point3D>) -> Vec<Point2D> {
    let mut new_points: Vec<Point2D> = Vec::new();
    let cameraz = 200.;
    let scale = 200.;
    for p in points {
        // println!("Converting Point {} {} {}", p.x, p.y, p.z);

        let z = p.z + cameraz;
        let x = (p.x / z) * scale;
        let y = (p.y / z) * scale;

        // println!("Converting Point {} {} ", x, y);

        new_points.push(Point2D { x: x, y: y });
    }

    return new_points;
}

fn draw_points(points: &Vec<Point2D>) {
    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;

    for p in points {
        // println!("Point {} {}", p.x, p.y);
        draw_circle(cx + p.x, cy + p.y, 1., WHITE);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut angle:f32 = 0.;
    let sphere_points = get_sphere_points();
    loop {
        clear_background(BLACK);

        angle += 1.;

        let rotated_points = rotate_x(&sphere_points, angle);
        let points = project(&rotated_points);
        draw_points(&points);
        next_frame().await
    }
}
