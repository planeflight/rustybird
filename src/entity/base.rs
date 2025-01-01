use raylib::{math::Rectangle, texture::Texture2D};

use crate::util::render_texture;

use super::traits::{PhysicsUpdate, Renderable};

pub struct Base {
    pub rect: Rectangle,
    pub texture: Texture2D,
    pub x: f32, // for scrolling
}

impl Renderable for Base {
    fn render(self: &Self, d: &mut raylib::prelude::RaylibDrawHandle) {
        // render 2 of these with a little scrolling action
        let mut rect1 = self.rect.clone();
        rect1.x = self.x;
        let mut rect2 = rect1.clone();
        rect2.x += rect2.width;
        render_texture(d, &self.texture, &rect1);
        render_texture(d, &self.texture, &rect2);
    }
}

impl PhysicsUpdate for Base {
    fn update(self: &mut Self, dt: f32, _: f32) {
        const VELOCITY: f32 = -400.;
        self.x += VELOCITY * dt;
        if self.x < -self.rect.width {
            self.x += self.rect.width;
        }
    }
}
