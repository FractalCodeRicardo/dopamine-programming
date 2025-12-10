
use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("MyGame")]
async fn main() {

    let map = vec![
        "##################",
        "# p  *   *  *    #",
        "# ############## #",
        "#   *  * *  * e  #",
        "# ############## #",
        "# ############## #",
        "#    *   *  *    #",
        "# ############## #",
        "#   *  * *  * e  #",
        "# ############## #",
        "# ############## #",
        "#    *   *  *    #",
        "# ############## #",
        "#   *  * *  * e  #",
        "##################"
    ];

    let mut player = vec2(0.,0.);
    let mut enemies = vec![];
    let mut  blocks = vec![];
    let mut food = vec![];

    for j in 0..map.len() {
        let l = map[j];
        for i in 0..l.len()  {
            let c = l
                .chars()
                .nth(i)
                .unwrap();


            let pos = vec2(i as f32, j as f32);
            if c == 'e' {
                enemies.push(pos);
            }

            if c == '*' {
                food.push(pos);
            }

            if c == 'p' {
                player = pos;
            }

            if c == '#' {
                blocks.push(pos);
            }
        }
    }

    let draw_rec = |v: &Vec2| {
        let size =40.;
        draw_rectangle(
            v.x * size, 
            v.y * size, 
            size, 
            size, BLUE
        );
    };


    let draw_cir = |v: &Vec2, color: Color, cir_size: f32| {
        let size =40.;
        draw_circle(
            (v.x * size) + size/2., 
            (v.y * size)+ size/2., 
            cir_size /2., 
            color
        );
    };

    let collides_block = |v: &Vec2| {
        blocks.iter().any(|i| i == v) 
    };

    let collides_enemy = || {
        enemies.iter().any(|i| i == &player)
    };

    let collides_food = |player: &Vec2, food: &mut Vec<Vec2>| {

        let pos = &player.clone();
        for i in 0..food.len() {
            let f = &food[i];
            if f == pos
            {
                food.remove(i);
                break;
            }
        }
    };

    loop {
        clear_background(BLACK);

        for f in &food {
            draw_cir(f, WHITE, 40.-20.);
        }

        for e in &enemies {
            draw_cir(e, RED, 40.-20.);
        }

        for b in &blocks {
            draw_rec(b);
        }

        draw_cir(&player, YELLOW, 40. -20.);

        let mut npos = player;
        if is_key_pressed(KeyCode::Left) { 
            npos = player + vec2(-1., 0.);
        }

        if is_key_pressed(KeyCode::Right) { 
            npos = player + vec2(1., 0.);
        }

        if is_key_pressed(KeyCode::Down) { 
            npos = player + vec2(0., 1.);
        }

        if is_key_pressed(KeyCode::Up) { 
            npos = player + vec2(0., -1.);
        }

        if !collides_block(&npos) {
            player = npos.clone();
        }

        collides_food(&player, &mut food);


        
        next_frame().await
    }
}
