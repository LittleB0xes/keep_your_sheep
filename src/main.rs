use std::collections::HashMap;

use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

use sprite_library::*;
mod sprite_library;

use sprite::Sprite;
mod sprite;

use entities::{Hero, Entity, Sheep};
mod entities;

struct Game {
    texture: Texture2D,
    atlas: HashMap<String, SpriteLibraryData>,
    scale: f32,

    entities: Vec<Box<dyn Entity>>,
}

impl Game {
    fn new() -> Self {
        rand::srand(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64);
        let texture = Texture2D::from_file_with_format(include_bytes!("../assets/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let atlas = read_atlas().unwrap();
        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        let hero = Hero::new(0.0, 0.0, &atlas);

        entities.push(Box::new(hero));
        for _i in 0..10 {
            let x = gen_range(0, 19) as f32 * 16.0;
            let y = gen_range(0, 10) as f32 * 16.0;
            let sheep = Sheep::new(x, y, &atlas);

            entities.push(Box::new(sheep));

        }

        
        Self {
            texture,
            atlas,
            scale: 4.0,
            entities,

        }
    }

    fn update(&mut self) {

        // Entites update
        for ent in self.entities.iter_mut() {
            ent.update();
        }

    }

    fn render(&mut self) {
        
        // Sort all element before displaying (depth sorting)
        self.entities.sort_by_key(|k| k.get_y());

        // ... and draw all the entities
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
