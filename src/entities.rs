use std::collections::HashMap;

use macroquad::math::{Rect, Vec2};
use macroquad::texture::Texture2D;

use crate::sprite_library::{self, SpriteLibraryData};
use crate::sprite::Sprite;

use crate::puppet_master::Behaviour;

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
    pub id: u32,
    pub entity_type: EntityType,
    pub position: Vec2,
    pub velocity: Vec2,
    pub max_speed: f32,
    pub direction: Vec2,
    sprite: Sprite,
    animations: HashMap<AnimationState, SpriteLibraryData>,
    animation_state: AnimationState,
    collision_box: Rect,
    pub behaviour: Behaviour,
    pub collidable: bool,
    pub thing_carried: Option<u32>,
    pub transporter: Option<u32>,
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
            behaviour: Behaviour::Playable,
            collidable: true,
            thing_carried: None,
            transporter: None,
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


    pub fn apply_direction(&mut self) {
        if self.direction != Vec2::ZERO {
            self.velocity = self.max_speed * self.direction;
        } else {
            self.velocity *= 0.8;
        }
    }

    pub fn apply_transporter_direction(&mut self, speed: f32) {
        if self.direction != Vec2::ZERO {
            self.velocity = speed * self.direction;
        } else {
            self.velocity *= 0.8;
        }
    }

    pub fn take(&mut self, id: u32) {
        self.thing_carried = Some(id);
    }
    pub fn taken_by(&mut self, id: u32) {
        self.behaviour = Behaviour::Transported;
        self.transporter = Some(id);
    }
    pub fn drop(&mut self) {
        self.thing_carried = None;
    }

    pub fn dropped(&mut self) {
        self.behaviour = Behaviour::FreeWalk;
        self.transporter = None;
    }

    pub fn motion(&mut self) {
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


fn sheep_incubator(sheep: &mut Entity) {
    sheep.max_speed = 0.5;
    sheep.behaviour = Behaviour::FreeWalk;
}



