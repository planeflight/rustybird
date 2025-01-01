use raylib::{
    math::{Rectangle, Vector2},
    prelude::RaylibDrawHandle,
    texture::Texture2D,
};

use crate::util::render_texture;

use super::traits::{PhysicsUpdate, Renderable};

pub struct Player {
    pub rect: Rectangle,
    pub texture1: Texture2D,
    pub texture2: Texture2D,
    pub texture3: Texture2D,
    pub velocity: Vector2,
}

impl Renderable for Player {
    fn render(self: &Self, d: &mut RaylibDrawHandle) {
        if self.velocity.y > 1.5 {
            render_texture(d, &self.texture3, &self.rect);
        } else if self.velocity.y < -1.5 {
            render_texture(d, &self.texture2, &self.rect);
        } else {
            render_texture(d, &self.texture1, &self.rect);
        }
    }
}

impl PhysicsUpdate for Player {
    fn update(self: &mut Self, dt: f32, gravity: f32) {
        self.velocity.y += gravity * dt;
        self.rect.y += self.velocity.y * dt;
    }
}

impl Player {
    pub fn jump(self: &mut Self) {
        self.velocity.y = -700.;
    }
}
