use std::collections::HashMap;

use macroquad::prelude::*;

use sprite_library::*;
mod sprite_library;

use sprite::Sprite;
mod sprite;

use entities::{Hero, Entity};
mod entities;

struct Game {
    texture: Texture2D,
    atlas: HashMap<String, SpriteLibraryData>,
    scale: f32,

    entities: Vec<Box<dyn Entity>>,
}

impl Game {
    fn new() -> Self {
        let texture = Texture2D::from_file_with_format(include_bytes!("../assets/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let atlas = read_atlas().unwrap();
        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        let hero = Hero::new(0.0, 0.0, &atlas);

        entities.push(Box::new(hero));

        Self {
            texture,
            atlas,
            scale: 4.0,
            entities,

        }
    }

    fn update(&mut self) {
        for ent in self.entities.iter_mut() {
            ent.update();
        }
    }

    fn render(&mut self) {
        for ent in self.entities.iter_mut() {
            ent.render(self.texture, self.scale);
        }
        
    }
}


#[macroquad::main(window_conf())]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();

        game.render();
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RuneLighter".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}
