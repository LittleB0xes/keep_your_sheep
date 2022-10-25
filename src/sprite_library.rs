use serde::Deserialize;
use std::collections::HashMap;

use crate::sprite::Sprite;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SpriteLibraryData {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub frame: i32,
    pub speed: i32,
}

pub fn read_atlas() -> HashMap<String, SpriteLibraryData>{

    let atlas: HashMap<String, SpriteLibraryData> = HashMap::from([
        ("hero_walk_right".to_string(), SpriteLibraryData{x: 0, y: 0, w: 16, h: 16, frame: 4, speed: 10}),
        ("hero_idle_right".to_string(), SpriteLibraryData{x: 0, y: 0, w: 16, h: 16, frame: 1, speed: 10}),
        ("hero_walk_left".to_string(), SpriteLibraryData{ x: 0, y: 16, w: 16, h: 16, frame: 4, speed: 10}),
        ("hero_idle_left".to_string(), SpriteLibraryData{ x: 0, y: 16, w: 16, h: 16, frame: 1, speed: 10 }),
        ("hero_walk_down".to_string(),  SpriteLibraryData{ x: 0, y: 48, w: 16, h: 16, frame: 4, speed: 10 }),
        ("hero_walk_up".to_string(), SpriteLibraryData{ x: 0, y: 32, w: 16, h: 16, frame: 4, speed: 10}),
        ("hero_idle_up".to_string(),  SpriteLibraryData{ x: 0, y: 32, w: 16, h: 16, frame: 1, speed: 10 }),
        ("hero_idle_down".to_string(), SpriteLibraryData{ x: 0, y: 48, w: 16, h: 16, frame: 1, speed: 10 }),
        ("sheep_walk_right".to_string(), SpriteLibraryData{ x: 64, y: 0, w: 16, h: 16, frame: 4, speed: 10 }),
        ("sheep_walk_left".to_string(), SpriteLibraryData{ x: 64, y: 16, w: 16, h: 16, frame: 4, speed: 10 }),
        ("sheep_idle_right".to_string(), SpriteLibraryData{ x: 64, y: 0, w: 16, h: 16, frame: 1, speed: 10 }),
        ("sheep_idle_left".to_string(), SpriteLibraryData{ x: 64, y: 16, w: 16, h: 16, frame: 1, speed: 10 }),
        ("sheep_idle_down".to_string(), SpriteLibraryData{ x: 64, y: 48, w: 16, h: 16, frame: 1, speed: 10 }),
        ("sheep_idle_up".to_string(), SpriteLibraryData{ x: 64, y: 32, w: 16, h: 16, frame: 1, speed: 10 }),
        ("sheep_walk_down".to_string(), SpriteLibraryData{ x: 64, y: 48, w: 16, h: 16, frame: 4, speed: 10 }),
        ("sheep_walk_up".to_string(), SpriteLibraryData{ x: 64, y: 32, w: 16, h: 16, frame: 4, speed: 10 }),
        ("wolf_walk_right".to_string(), SpriteLibraryData{ x: 128, y: 0, w: 32, h: 16, frame: 8, speed: 5 }),
        ("wolf_walk_left".to_string(), SpriteLibraryData{ x: 128, y: 16, w: 32, h: 16, frame: 8, speed: 5 }),
        ("wolf_idle_right".to_string(), SpriteLibraryData{ x: 128, y: 32, w: 32, h: 16, frame: 5, speed: 10 }),
        ("wolf_idle_left".to_string(), SpriteLibraryData{ x: 128, y: 48, w: 32, h: 16, frame: 5, speed: 10 }),
        ("wolf_sleep_right".to_string(), SpriteLibraryData{ x: 448, y: 32, w: 32, h: 16, frame: 6, speed: 10 }),
        ("wolf_sleep_right".to_string(), SpriteLibraryData{ x: 448, y: 48, w: 32, h: 16, frame: 6, speed: 10 })
    ]);
    atlas
}
