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
        let texture = Texture2D::from_file_with_format(include_bytes!("../assets/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let ground_texture = Texture2D::from_file_with_format(include_bytes!("../assets/sheep/simplified/Level_0/Ground.png"), None);
        ground_texture.set_filter(FilterMode::Nearest);

        let level = Level::new();

        let mut id_counter = 0;

        let atlas = read_atlas().unwrap();
        let mut entities = Vec::new();
        let hero = Entity::new(10.0, 0.0, EntityType::Hero, id_counter, &atlas);

        entities.push(hero);

        // create a vec to store all places already taked by a sheep
        let mut entities_grid: Vec<bool> = vec![true; level.cell_w * level.cell_h];
        for _i in 0..20 {
            id_counter += 1;
            let mut free_place = false;
            let mut x: usize = 0;
            let mut y: usize = 0;

            // Check if the place is free
            while !free_place {
                x = gen_range(0, 26);
                y = gen_range(0, 15);
                if level.get_int_at(x, y) == 0 && entities_grid[x + y * level.cell_w] {
                    free_place = true;
                }
            }
            entities_grid[x + y * level.cell_w] = false;
            let sheep = Entity::new((x * 16) as f32, (y * 16) as f32, EntityType::Sheep, id_counter, &atlas);

            entities.push(sheep);
        }
        entities.push(Entity::new(10.0, 100.0, EntityType::Wolf, id_counter, &atlas));

        Self {
            //id_counter,
            texture,
            level,
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
        puppet_master::play(&mut self.entities, &self.level);
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
        draw_text(&format!("{}", get_fps()), 30.0, 30.0, 24.0, RED);
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
