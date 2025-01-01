mod entity;
mod util;

use std::rc::Rc;

use entity::{
    base::Base,
    pipe::{new_pipe_set, Pipe},
    player::Player,
    traits::{PhysicsUpdate, Renderable},
};
use raylib::prelude::*;
use util::{get_high_score, load_texture, render_texture, update_high_score};

const WIDTH: i32 = 720;
const HEIGHT: i32 = 1080;

struct GameData {
    player: Player,
    pipes: Vec<Pipe>,
    base: Base,
    score: i32,
    high_score: i32,
}

impl GameData {
    fn reset(self: &mut Self) {
        self.pipes.clear();
        self.player.rect.y = (HEIGHT as f32) * 0.375 - 24.;
        self.player.velocity.y = 0.;

        update_high_score(self.score / 2);
        self.high_score = get_high_score();
        self.score = 0;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("FLAPPY")
        .vsync()
        .build();
    let mut game: GameData = GameData {
        player: Player {
            rect: raylib::math::Rectangle {
                x: (WIDTH as f32) / 2. - 24.,
                y: (HEIGHT as f32) * 0.375 - 24.,
                width: 48.,
                height: 48.,
            },
            texture1: load_texture(&mut rl, &thread, "./res/flappy_midflap.png"),
            texture2: load_texture(&mut rl, &thread, "./res/flappy_downflap.png"),
            texture3: load_texture(&mut rl, &thread, "./res/flappy_upflap.png"),
            velocity: Vector2 { x: 0., y: 0. },
        },
        pipes: vec![],
        base: Base {
            rect: Rectangle {
                x: 0.,
                y: HEIGHT as f32 * 0.75,
                width: WIDTH as f32,
                height: HEIGHT as f32 * 0.25,
            },
            texture: load_texture(&mut rl, &thread, "./res/base.png"),
            x: 0.,
        },
        score: 0,
        high_score: get_high_score(),
    };
    const GRAVITY: f32 = 2000.;

    let background = load_texture(&mut rl, &thread, "./res/background_night.png");

    let top_pipe_texture = Rc::new(load_texture(&mut rl, &thread, "./res/pipe_top.png"));
    let bottom_pipe_texture = Rc::new(load_texture(&mut rl, &thread, "./res/pipe_bottom.png"));

    const SPAWN_TIME: f32 = 1.2;
    let mut timer = 0.;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let dt: f32 = rl.get_frame_time();
        timer += dt;
        // input
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game.player.jump();
        }

        // update
        game.player.update(dt, GRAVITY);

        // spawn new pipes
        if timer >= SPAWN_TIME {
            timer -= SPAWN_TIME;
            new_pipe_set(
                &mut game.pipes,
                top_pipe_texture.clone(),
                bottom_pipe_texture.clone(),
                WIDTH as f32 * 1.5,
                rand::random::<f32>() * 400. + 200.,
            );
        }

        // update the pipes
        let mut should_reset = false;
        for pipe in game.pipes.iter_mut() {
            // move pipes
            pipe.update(dt, GRAVITY);
            // check collision
            if pipe.rect.check_collision_recs(&game.player.rect) {
                should_reset = true;
                break;
            }
            // check if passed player
            if pipe.passed_player(&game.player.rect) {
                game.score += 1;
            }
        }
        game.base.update(dt, GRAVITY);
        should_reset = should_reset || game.base.rect.check_collision_recs(&game.player.rect);

        if should_reset {
            game.reset();
        }
        // delete the dead pipes
        game.pipes.retain(|p: &Pipe| !p.dead());

        // borrow render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        // render background
        render_texture(
            &mut d,
            &background,
            &Rectangle {
                x: 0.,
                y: 0.,
                width: WIDTH as f32,
                height: HEIGHT as f32,
            },
        );
        // render pipes
        for pipe in game.pipes.iter_mut() {
            pipe.render(&mut d);
        }
        game.base.render(&mut d);
        game.player.render(&mut d);

        // render score
        let actual_score = game.score / 2;
        let text = actual_score.to_string();
        let font_size = 48;
        d.draw_text(
            &text.to_string(),
            WIDTH / 2 - d.measure_text(&text.to_string(), 48) / 2,
            HEIGHT / 4,
            font_size,
            Color::WHITE,
        );

        // render high score
        let mut htext = String::from("High Score: ");
        htext.push_str(game.high_score.to_string().as_str());
        d.draw_text(htext.as_str(), 10, 10, 32, Color::WHITE);
    }
}
