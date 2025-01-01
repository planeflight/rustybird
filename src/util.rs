use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

pub fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Texture2D {
    let texture = rl
        .load_texture(thread, path)
        .expect(format!("Error loading '{}'", path).as_str());
    return texture;
}

pub fn render_texture(d: &mut RaylibDrawHandle, texture: &Texture2D, dest: &Rectangle) {
    d.draw_texture_pro(
        texture,
        Rectangle {
            x: 0.,
            y: 0.,
            width: texture.width as f32,
            height: texture.height as f32,
        },
        *dest,
        Vector2 { x: 0., y: 0. },
        0.,
        Color::WHITE,
    );
}

pub fn read_file() -> String {
    let path = Path::new("./res/save.flappy");

    let mut file = match File::open(&path) {
        Err(why) => panic!("Error: Failed to open '{}': {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Error: Failed to read '{}': {}", path.display(), why),
        Ok(_) => return s,
    }
}

pub fn get_high_score() -> i32 {
    let string = read_file();
    return string.trim().to_string().parse::<i32>().unwrap();
}

pub fn update_high_score(new_score: i32) {
    let current_best = get_high_score();
    // update if new > current
    if new_score > current_best {
        fs::write("./res/save.flappy", new_score.to_string().as_bytes())
            .expect("Unable to write file");
    }
}
