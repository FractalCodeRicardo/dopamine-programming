use core::time;
use std::thread;

const SCREEN_SIZE: usize = 50;
const SPHERE_WIDTH: usize = 20;
const SPHERE_HEIGTH: usize = 15;
const STEP: f32 = 5.;

struct P {
    x: f32,
    y: f32,
    z: f32,
}

struct V {
    x: f32,
    y: f32,
    symbol: char
}

fn points() -> Vec<P> {
    let mut points = vec![];
    let mut t = 0.;
    let mut p = 0.;

    while t <= 360. {
        p = 0.;
        while p <= 360. {
            let tr = (t as f32).to_radians();
            let pr = (p as f32).to_radians();

            let x = tr.cos() * pr.sin();
            let y = tr.sin() * pr.sin();
            let z = pr.cos();

            points.push(P { x, y, z });

            p += STEP;
        }

        t += STEP;
    }

    return points;
}
fn distance(p1: &P, p2: &P) -> f32 {
    let x = p1.x - p2.x;
    let y = p1.y - p2.y;
    let z = p1.z - p2.z;

    let mut distance = (x*x) + (y*y) +(z*z);
    distance = distance.sqrt();
    distance
}
fn get_symbol(point:  &P, camera: &P) -> char {

    let dist = distance(point, camera);
    let max_dist = distance(camera, &P{x: 0., y:0. , z:-1.});


    let symbols = vec!['#', '@', 'r', 's', '.', ',', ' '];

    let len =(symbols.len() -1) as f32;
    let portion = (dist / max_dist).clamp(0., 1.);
    let index = (portion * len) as usize;
    return symbols[index];
}

fn to_screen_2d(points_3d: &Vec<P>) -> Vec<V> {
    let mut points_2d = vec![];
    let camera = P { x:0., y:0., z:1.5};

    for p in points_3d {
        let mut x = p.x / (p.z + camera.z);
        let mut y = p.y / (p.z + camera.z);

        x = (p.x + 1.) * SPHERE_WIDTH as f32;
        y = (p.y + 1.) * SPHERE_HEIGTH as f32;

        x += 5.;
        y += 5.;

        let symbol = get_symbol(&p, &camera);
        points_2d.push(V { x, y, symbol });
    }

    return points_2d;
}

fn rotate_xy(points: &Vec<P>, ry: f32, rx: f32) -> Vec<P>{
    let cosy = ry.cos();
    let siny = ry.sin();


    let cosx = rx.cos();
    let sinx = rx.sin();
    let mut new_points = vec![];
    for p in points {
        let x1 = p.x * cosy + p.z * siny;
        let y1 = p.y;
        let z1 = -p.x * siny + p.z * cosy;


        let y2 = y1 * cosx -z1 * sinx;
        let z2 = y1 * sinx + z1 * cosx;
        let x2 = x1;

        new_points.push(P {
            x: x2, y: y2, z:z2
        })
    }

    return new_points
}

fn print_screen(points: &Vec<V>) {
    let mut screen: Vec<Vec<char>> = vec![vec![' '; SCREEN_SIZE]; SCREEN_SIZE];

    for p in points {
        let x = p.x as usize;
        let y = p.y as usize;

        // println!("{} {}", x, y);
        screen[y][x] = p.symbol;
    }

    for line in screen {
        let line_chars: String = line.iter().collect();
        println!("{line_chars}");
    }
}

fn main() {
    let points = points();
    let mut radians_y = 0.;
    let mut radians_x = 0.;
    loop {
        print!("\x1B[2J\x1B[H");
        let rotated_points = rotate_xy(&points, radians_y, radians_x);
        let points_screen = to_screen_2d(&rotated_points);
        print_screen(&points_screen);

        radians_x += 0.1;
        radians_y += 0.2;
        thread::sleep(time::Duration::from_millis(33));
    }
}
