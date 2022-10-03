use std::collections::HashMap;

use crate::sprite_library::{self, SpriteLibraryData};
use macroquad::input::*;
use macroquad::math::{Rect, Vec2};
use macroquad::texture::Texture2D;

use crate::sprite::Sprite;


use super::{EntityType, AnimationState, Entity};

pub struct Hero {
    entity_type: EntityType,
    position: Vec2,
    velocity: Vec2,
    max_speed: f32,
    direction: Vec2,
    sprite: Sprite,
    animations: HashMap<AnimationState, SpriteLibraryData>,
    animation_state: AnimationState,
    collision_box: Rect,
}

impl Hero {
    pub fn new(x: f32, y: f32, atlas: &HashMap<String, SpriteLibraryData>) -> Self {
        let mut animations = HashMap::new();
        for anim in [
            (AnimationState::WalkSide, "hero_walk_right"),
            (AnimationState::WalkUp, "hero_walk_up"),
            (AnimationState::WalkDown, "hero_walk_down"),
        ]
        .iter()
        {
            animations.insert(
                anim.0,
                sprite_library::extract_data(atlas, anim.1.to_string()),
            );
        }

        let animation_state = AnimationState::WalkUp;
        let mut sprite = Sprite::new(*animations.get(&animation_state).unwrap());
        sprite.set_position_to(Vec2::new(x, y));

        Self {
            entity_type: EntityType::Hero,
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            max_speed: 1.0,
            direction: Vec2::ZERO,
            animations: HashMap::new(),
            animation_state,
            sprite: Sprite::new(*animations.get(&animation_state).unwrap()),
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
        } else {
            self.velocity *= 0.9;
        }

        self.position += self.velocity;

        self.sprite.set_position_to(self.position);
    }

    fn get_collision_box(&self) -> Rect {
        self.collision_box.offset(self.position)

    }
}
