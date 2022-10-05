use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

use sprite_library::*;
mod sprite_library;

mod puppet_master;
mod sprite;

use entities::{Entity, EntityType};
mod entities;

use level::Level;
mod level;

struct Game {
    //id_counter: u32,
    level: Level,
    texture: Texture2D,
    ground_texture: Texture2D,
    //atlas: HashMap<String, SpriteLibraryData>,
    scale: f32,

    entities: Vec<Entity>,
}

impl Game {
    fn new() -> Self {
        srand(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        );
        let texture =
            Texture2D::from_file_with_format(include_bytes!("../assets/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let ground_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sheep/simplified/Level_0/Ground.png"), None);
        ground_texture.set_filter(FilterMode::Nearest);

        let mut id_counter = 0;

        let atlas = read_atlas().unwrap();
        let mut entities = Vec::new();
        let hero = Entity::new(10.0, 10.0, EntityType::Hero, id_counter, &atlas);

        entities.push(hero);
        for _i in 0..10 {
            id_counter += 1;
            let x = gen_range(0, 26) as f32 * 16.0;
            let y = gen_range(0, 15) as f32 * 16.0;
            let sheep = Entity::new(x, y, EntityType::Sheep, id_counter, &atlas);

            entities.push(sheep);
        }

        Self {
            //id_counter,
            level: Level::new("Level_0".to_string()),
            texture,
            ground_texture,
            //atlas,
            scale: 3.0,
            entities,
        }
    }

    //fn id_generator( &mut self) -> u32 {
    //    self.id_counter += 1;
    //    self.id_counter
    //}

    fn update(&mut self) {
        puppet_master::play(&mut self.entities);
    }

    fn render(&mut self) {

        // Draw ground
        self.level.render(self.ground_texture, self.scale);
        
        // Sort all element before displaying (depth sorting)
        self.entities.sort_by_key(|k| k.depth_sort());

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
        draw_text(&format!("{}", get_fps()), 30.0, 30.0, 16.0, WHITE);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Keep Your Sheep!".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}
