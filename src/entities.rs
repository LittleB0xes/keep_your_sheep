
use std::collections::HashMap;

use crate::sprite_library::{self, SpriteLibraryData};
use macroquad::input::*;
use macroquad::math::{Rect, Vec2};
use macroquad::texture::Texture2D;


use crate::sprite::Sprite;

pub enum EntityType {
    Hero,
    Sheep
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum AnimationState {
    WalkSide,
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown
}

pub struct Entity {
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

impl Entity {
    pub fn new(x: f32, y: f32, entity_type: EntityType, atlas: &HashMap<String, SpriteLibraryData>) -> Self {
        let animations = set_animation(&entity_type, atlas);
        let animation_state = AnimationState::WalkUp;
        let mut sprite = Sprite::new(*animations.get(&animation_state).unwrap());
        sprite.set_position_to(Vec2::new(x, y));

        Self {
            entity_type,
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            max_speed: 1.0,
            direction: Vec2::ZERO,
            animations,
            animation_state,
            sprite,
            collision_box: Rect::new(0.0, 0.0, 16.0, 16.0),
        }
    }
    pub fn render(&mut self, texture: Texture2D, scale: f32) {
        self.sprite.draw_sprite(texture, scale);
    }

    pub fn update(&mut self) {
        match self.entity_type {
            EntityType::Hero =>{update_hero(self)},
            EntityType::Sheep => {update_sheep(self)},
        }

        self.animation_manager();
    }

    pub fn animation_manager(&mut self) {
        if self.direction.y > 0.0 && self.animation_state != AnimationState::WalkDown {
            self.animation_state = AnimationState::WalkDown;
            self.sprite.set_animation(&self.animations.get(&AnimationState::WalkDown).unwrap());
            self.sprite.play();

        }
        else if self.direction.y < 0.0 && self.animation_state != AnimationState::WalkUp {
            self.animation_state = AnimationState::WalkUp;
            self.sprite.set_animation(&self.animations.get(&AnimationState::WalkUp).unwrap());
            
            self.sprite.play();
            
        }
        
        if self.direction.x > 0.0 && self.animation_state != AnimationState::WalkRight {
            self.animation_state = AnimationState::WalkRight;
            self.sprite.set_animation(&self.animations.get(&AnimationState::WalkSide).unwrap());
            self.sprite.flip_x = false;
            self.sprite.play();

        }
        else if self.direction.x < 0.0 && self.animation_state != AnimationState::WalkLeft {
            self.animation_state = AnimationState::WalkLeft;
            self.sprite.set_animation(&self.animations.get(&AnimationState::WalkSide).unwrap());
            self.sprite.flip_x = true;
            
            self.sprite.play();
        }
        else if self.direction == Vec2::ZERO {
            self.sprite.stop();

        }

    }

    pub fn get_collision_box(&self) -> Rect {
        self.collision_box.offset(self.position)
    }

    pub fn get_y(&self) -> u32 {
        self.position.y as u32
            
    }
}


fn set_animation(entity_type: &EntityType, atlas: &HashMap<String, SpriteLibraryData>) -> HashMap<AnimationState, SpriteLibraryData>{
    let mut animations = HashMap::new();
    let list = match entity_type {
        EntityType::Hero => [(AnimationState::WalkSide, "hero_walk_right"), (AnimationState::WalkUp, "hero_walk_up"), (AnimationState::WalkDown, "hero_walk_down")],
        EntityType::Sheep => [(AnimationState::WalkSide, "sheep_walk"), (AnimationState::WalkUp, "sheep_walk_up"), (AnimationState::WalkDown, "sheep_walk_down")],
    };

    for anim in list.iter(){
        animations.insert(
            anim.0,
            sprite_library::extract_data(atlas, anim.1.to_string()),
        );
    }

    animations

}


fn update_hero(hero: &mut Entity) {
        hero.direction.x = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -1.0,
            (false, true) => 1.0,
        };

        hero.direction.y = match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -1.0,
            (false, true) => 1.0,
        };

        if hero.direction != Vec2::ZERO {
            hero.velocity = hero.max_speed * hero.direction;
        } else {
            hero.velocity *= 0.8;
        }

        hero.position += hero.velocity;

        hero.sprite.set_position_to(hero.position);
}



fn update_sheep(sheep: &mut Entity) {}



