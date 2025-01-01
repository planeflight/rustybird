use std::rc::Rc;

use raylib::{math::Rectangle, texture::Texture2D};

use crate::util::render_texture;

use super::traits::{PhysicsUpdate, Renderable};

pub struct Pipe {
    pub rect: Rectangle,
    pub texture: Rc<Texture2D>,
    pub active: bool,
}

impl Renderable for Pipe {
    fn render(self: &Self, d: &mut raylib::prelude::RaylibDrawHandle) {
        render_texture(d, &self.texture, &self.rect);
    }
}

impl PhysicsUpdate for Pipe {
    fn update(self: &mut Self, dt: f32, _: f32) {
        const VELOCITY: f32 = -400.;
        self.rect.x += VELOCITY * dt;
    }
}

impl Pipe {
    pub fn dead(self: &Self) -> bool {
        return self.rect.x + self.rect.width < 0.;
    }
    pub fn passed_player(self: &mut Self, p: &Rectangle) -> bool {
        let passed: bool = self.rect.x + self.rect.width < p.x;
        let increase_score: bool = passed && self.active;
        if passed {
            self.active = false;
        }
        return increase_score;
    }
}

pub fn new_pipe_set(
    pipes: &mut Vec<Pipe>,
    top_pipe_texture: Rc<Texture2D>,
    bottom_pipe_texture: Rc<Texture2D>,
    x: f32,
    height: f32,
) {
    let width = 100.;
    let gap = width * 2.1;
    let scaled_height = width * top_pipe_texture.height as f32 / top_pipe_texture.width as f32;

    // top
    pipes.push(Pipe {
        rect: Rectangle {
            x,
            y: height - scaled_height - gap / 2.0,
            width,
            height: scaled_height,
        },
        texture: top_pipe_texture,
        active: true,
    });
    // bottom
    pipes.push(Pipe {
        rect: Rectangle {
            x,
            y: height + gap / 2.0,
            width,
            height: scaled_height,
        },
        texture: bottom_pipe_texture,
        active: true,
    });
}
