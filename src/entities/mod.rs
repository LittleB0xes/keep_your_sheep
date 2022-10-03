use std::collections::HashMap;


use macroquad::math::{Vec2, Rect};
use macroquad::texture::Texture2D;
use macroquad::input::*;
use crate::sprite_library::{SpriteLibraryData, self};

use crate::sprite::Sprite;


pub trait Entity {
    fn render(&mut self, texture: Texture2D, scale: f32);
    fn update(&mut self);
}

enum EntityType {
    Hero,
}


pub struct Hero {
    entity_type: EntityType,
    position: Vec2,
    velocity: Vec2,
    max_speed: f32,
    direction: Vec2,
    sprite: Sprite,
    collision_box: Rect,
}

impl Hero {
    pub fn new(x: f32, y: f32, atlas: &HashMap<String, SpriteLibraryData>) -> Self {

        Self {
            entity_type: EntityType::Hero,
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            max_speed: 1.0,
            direction: Vec2::ZERO,
            sprite: Sprite::new(sprite_library::extract_data(atlas, "hero_walk_down".to_string())),
            collision_box: Rect::new(0.0, 0.0, 16.0, 16.0),
        }
    }
}


impl Entity for Hero {

    fn render(&mut self, texture: Texture2D, scale: f32) {
        self.sprite.draw_sprite(texture, scale);
    }

    fn update(&mut self) {
        self.direction.x = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -1.0,
            (false, true) => 1.0,
        };

        self.direction.y = match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -1.0,
            (false, true) => 1.0,
        };

        if self.direction != Vec2::ZERO {
            self.velocity = self.max_speed * self.direction;

        }
        else {
            self.velocity *= 0.9;
        }


        self.position += self.velocity;

        self.sprite.set_position_to(self.position);
    }


}


