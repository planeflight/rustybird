use raylib::prelude::RaylibDrawHandle;

pub trait Renderable {
    fn render(self: &Self, d: &mut RaylibDrawHandle);
}

pub trait PhysicsUpdate {
    fn update(self: &mut Self, dt: f32, gravity: f32);
}
