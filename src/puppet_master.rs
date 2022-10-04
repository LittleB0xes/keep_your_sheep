use macroquad::input::*;
use macroquad::math::Vec2;
use macroquad::rand::gen_range;

use crate::entities::Entity;

pub fn play(entities: &mut Vec<Entity>) {
    // Apply each entity's behaviours
    //for ent in entities.iter_mut() {
    for i in 0..entities.len() {
        let mut ent = entities[i].clone();
        match ent.behaviour {
            Behaviour::Playable => playable(&mut ent, entities),
            Behaviour::FreeWalk => free_walk(&mut ent),
            Behaviour::Transported => transported(&mut ent, entities),
        }
        // Replace by the new updated entity
        entities[i] = ent;
    }

    // Check collision between entities
    entity_entity_collision(entities);

    // Make all entities move
    motion(entities);
}

pub fn entity_entity_collision(entities: &mut Vec<Entity>) {
    // Collision detection
    for i in 0..entities.len() {
        let mut ent = entities[i].clone();
        for j in 0..entities.len() {
            if ent.collidable
                && entities[j].collidable
                && ent.id != entities[j].id
                && ent
                    .get_collision_box()
                    .overlaps(&entities[j].get_collision_box())
            {
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
#[derive(Clone, PartialEq)]
pub enum Behaviour {
    Playable,
    FreeWalk,
    Transported,
}

fn playable(ent: &mut Entity, entities: &mut Vec<Entity>) {
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


    // Check if entity take somthing or drop something
    if is_key_pressed(KeyCode::Space) {
        match ent.thing_carried {
            Some(id) => {
                    println!("carried {:?}", ent.thing_carried);
                    for other in entities.iter_mut() {
                        if other.id == id {
                        other.dropped();
                        ent.drop();
                        }
                    }
                    
                },
            None => {
                for other in entities.iter_mut() {
                    let dist = (ent.get_collision_box().center() - other.get_collision_box().center()).length_squared();
                    if other.id != ent.id && dist < 100.0 {
                        ent.take(other.id);
                        other.taken_by(ent.id);
                    }
                }
            }
                    
        }
    }
   

    ent.apply_direction();
}

fn free_walk(ent: &mut Entity) {
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

fn transported(ent: &mut Entity, entities: &mut Vec<Entity>) {
    let mut speed: f32 = 0.0;
    for other in entities.iter() {
        if other.id == ent.transporter.unwrap() {
            ent.direction = other.direction;
            speed = other.max_speed;

        }
    }
    ent.apply_transporter_direction(speed);

}
