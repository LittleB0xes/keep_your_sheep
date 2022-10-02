use std::collections::HashMap;

use macroquad::prelude::*;

use sprite_library::*;
mod sprite_library;

use sprite::Sprite;
mod sprite;

struct Game {
    texture: Texture2D,
    atlas: HashMap<String, SpriteLibraryData>,
    scale: f32,

    hero: Sprite,
}

impl Game {
    fn new() -> Self {
        let texture = Texture2D::from_file_with_format(include_bytes!("../assets/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let atlas = read_atlas().unwrap();
        let hero = Sprite::new(extract_data(&atlas, "hero_walk_right".to_string()));

        Self {
            texture,
            atlas,
            scale: 4.0,
            hero,

        }
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
        self.hero.draw_sprite(self.texture, self.scale);
        
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
        //high_dpi: true,
        ..Default::default()
    }
}
