use macroquad::math::Vec2;
use macroquad::input::*;
use macroquad::rand::gen_range;

use crate::entities::Entity;

pub fn play(entities: &mut Vec<Entity>) {
    // Apply each entity's behaviours
    for ent in entities.iter_mut() {
        match ent.behaviour {
            Behaviour::Playable => playable(ent),
            Behaviour::FreeWalk => free_walk(ent),
            _ => {},
        }
    }

    // Check collision between entities
    entity_entity_collision(entities);
    
    // Make all entities move
    motion(entities);
}


pub fn entity_entity_collision(entities: &mut Vec<Entity>) {
        // Collision detection
        for i  in 0..entities.len() {
            let mut ent = entities[i].clone();
            for j in 0..entities.len() {
                if ent.id != entities[j].id && ent.get_collision_box().overlaps(&entities[j].get_collision_box()){
                    ent.direction = Vec2::ZERO;
                    ent.velocity = Vec2::ZERO
                }   
            }
            // Replace by the new updated entity
            entities[i] = ent;    
        }
}

pub fn motion(entities: &mut Vec<Entity>) {
    for ent in entities.iter_mut() {
            ent.motion();
    }
}


// Behaviours
#[derive(Clone)]
pub enum Behaviour {
    Playable,
    FreeWalk,
    Transported,
}


pub fn playable(ent: &mut Entity){
         ent.direction.x = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -1.0,
            (false, true) => 1.0,
        };

        ent.direction.y = match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -1.0,
            (false, true) => 1.0,
        };
        
        ent.apply_direction();
}


pub fn free_walk(ent: &mut Entity) {
    if gen_range(0, 100) < 2 {
        let alea = gen_range(0, 6);
        match alea {
            0 => ent.direction = Vec2::new(0.0, 1.0),
            1 => ent.direction = Vec2::new(0.0, -1.0),
            2 => ent.direction = Vec2::new(1.0, 0.0),
            3 => ent.direction = Vec2::new(-1.0, 0.0),
            _ => ent.direction = Vec2::ZERO,
            
        }
    }

    ent.apply_direction();

}