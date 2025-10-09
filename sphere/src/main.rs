use macroquad::prelude::*;

const RADIUS: f32 = 200.;
const DEGREES: f32= 10.;

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

struct Point2D {
    x: f32,
    y: f32,
}

fn get_points() -> Vec<Point3D> {
    let mut theta: f32 = 0.;
    let mut phi: f32 = 0.;
    let mut points: Vec<Point3D> = Vec::new();

    while theta <= 360. {

        phi = 0.;
        while phi <= 360. {

            let p = get_point(theta.to_radians(), phi.to_radians());
            points.push(p);
            phi += DEGREES;
        }

        theta += DEGREES;
    }

    return points;
}

fn get_point(theta: f32, phi: f32) -> Point3D {
    let x = RADIUS * theta.sin() * phi.cos();
    let y = RADIUS * theta.sin() * phi.sin();
    let z = RADIUS * theta.cos();

    return Point3D { x: x, y: y, z: z };
}

fn project(points: &Vec<Point3D>) -> Vec<Point2D> {

    let cameraz = 500.;
    let scale = 200.;

    let mut new_points: Vec<Point2D> = Vec::new();
    for p in points {

        // println!("point {} {} {}", p.x, p.y, p.z);

        let z = p.z + cameraz;
        let y = (p.y/z) * scale;
        let x = (p.x/z) * scale;

        new_points.push(Point2D {
            x: x,
            y: y
        })
    }

    return new_points;

}


fn rotate(points: &Vec<Point3D>, angle: f32) -> Vec<Point3D> {

    let mut new_points: Vec<Point3D> = Vec::new();
    for p in points {

        let x = p.x;
        let y = p.y * angle.cos() - p.z * angle.sin();
        let z = p.y * angle.sin() + p.z * angle.cos();

        new_points.push(Point3D {
            x: x,
            y: y,
            z: z

        });
    }

    return new_points;
}

fn draw(points: &Vec<Point2D>){

    let cx = screen_width() / 2.;
    let cy = screen_height() / 2.;
    for p in points {
        draw_circle(cx + p.x, cy + p.y, 2., MAGENTA);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let sphere_points = get_points();
    let mut angle: f32 = 0.;
    loop {
        clear_background(BLACK);

        let rotate_points = rotate(&sphere_points, angle.to_radians());
        let points = project(&rotate_points);

        draw(&points);

        angle += 5.;
        next_frame().await
    }
}
