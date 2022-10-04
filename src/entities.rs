use std::collections::HashMap;

use macroquad::input::*;
use macroquad::math::{Rect, Vec2};
use macroquad::rand::gen_range;
use macroquad::texture::Texture2D;

use crate::sprite_library::{self, SpriteLibraryData};
use crate::sprite::Sprite;


#[derive(Copy, Clone)]
pub enum EntityType {
    Hero,
    Sheep
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum AnimationState {
    Idle,
    WalkSide,
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown
}

#[derive(Clone)]
pub struct Entity {
    id: u32,
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
    pub fn new(x: f32, y: f32, entity_type: EntityType, id: u32, atlas: &HashMap<String, SpriteLibraryData>) -> Self {
        let animations = set_animation(&entity_type, atlas);
        let animation_state = AnimationState::WalkUp;
        let mut sprite = Sprite::new(*animations.get(&animation_state).unwrap());
        sprite.set_position_to(Vec2::new(x, y));

        
        let mut entity = Entity {
            id,
            entity_type,
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            max_speed: 1.0,
            direction: Vec2::ZERO,
            animations,
            animation_state,
            sprite,
            collision_box: Rect::new(2.0, 10.0, 12.0, 6.0),
        };

        match entity_type {
            EntityType::Sheep => sheep_incubator(&mut entity),
            EntityType::Hero => {},
            
        }

        entity
    }

    pub fn render(&mut self, texture: Texture2D, scale: f32) {
        self.sprite.draw_sprite(texture, scale);
    }

    pub fn update(&mut self, entities: &mut Vec<Entity>) {
        match self.entity_type {
            EntityType::Hero =>{update_hero(self, entities)},
            EntityType::Sheep => {update_sheep(self, entities)},
        }

        if self.direction != Vec2::ZERO {
            self.velocity = self.max_speed * self.direction;
        } else {
            self.velocity *= 0.8;
        }
        
        // Collision detection
        for ent in entities.iter_mut() {
            if self.id != ent.id && self.get_collision_box().overlaps(&ent.get_collision_box()){
                self.direction = Vec2::ZERO;
                self.velocity = Vec2::ZERO
            }
        }

    
        self.position += self.velocity;
    
        self.sprite.set_position_to(self.position);

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
            self.animation_state =  AnimationState::Idle;
            self.sprite.stop();
        }

    }

    pub fn get_collision_box(&self) -> Rect {
        self.collision_box.offset(self.position + self.velocity + self.velocity)
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


fn update_hero(hero: &mut Entity, entities: &mut Vec<Entity>) {
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
        

}

fn sheep_incubator(sheep: &mut Entity) {
    sheep.max_speed = 0.5;
}

fn update_sheep(sheep: &mut Entity, entities: &mut Vec<Entity>) {
    if gen_range(0, 100) < 2 {
        let alea = gen_range(0, 6);
        match alea {
            0 => sheep.direction = Vec2::new(0.0, 1.0),
            1 => sheep.direction = Vec2::new(0.0, -1.0),
            2 => sheep.direction = Vec2::new(1.0, 0.0),
            3 => sheep.direction = Vec2::new(-1.0, 0.0),
            _ => sheep.direction = Vec2::ZERO,
            
        }
    }
}



