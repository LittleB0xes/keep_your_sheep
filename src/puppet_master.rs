use macroquad::input::*;
use macroquad::math::Vec2;
use macroquad::rand::gen_range;

use crate::entities::Entity;


/// the main puppet_master's function
pub fn play(entities: &mut Vec<Entity>) {
    // Apply each entity's behaviours
    for i in 0..entities.len() {
        let mut ent = entities[i].clone();
        match ent.behaviour {
            Behaviour::Playable => playable(&mut ent, entities),
            Behaviour::FreeWalk => free_walk(&mut ent),
            Behaviour::Transported => transported(&mut ent, entities),
            Behaviour::Thrown {dir, h, thrower} => thrown(&mut ent, dir, h, thrower),
        }
        // Replace by the new updated entity
        entities[i] = ent;
    }

    // Check collision between entities
    entity_entity_collision(entities);

    // Make all entities move
    motion(entities);
}


/// Check collision between entities
pub fn entity_entity_collision(entities: &mut Vec<Entity>) {
    // Collision detection
    for i in 0..entities.len() {
        let mut ent = entities[i].clone();
        for j in 0..entities.len() {


            //let avoid_collision = match (ent.thing_carried, entities[j].transporter) {
            //    (Some(a), Some(b)) => {
            //        if a == entities[j].id || ent.id == b {true} else {false}
            //    },

            //    _ => false
            //};

            // Avoid collison if transported or with a transported thing
            let avoid_collision = match (ent.behaviour, entities[j].behaviour) {
                (_, Behaviour::Transported)
                | (Behaviour::Transported, _) => { true },
                _ => false
            };

            let avoid_thrower = match (ent.behaviour, entities[j].behaviour) {
                (Behaviour::Thrown {dir, h, thrower}, _)
                | (_, Behaviour::Thrown {dir, h, thrower}) => {
                    if ent.id == thrower || entities[j].id == thrower {true} else {false}
                }

                _ => false,
                
            };



            if ent.collidable
                && !avoid_collision
                && !avoid_thrower
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

/// Behaviours enum
/// 
/// Playable: for entity controlled by a player
/// FreeWalk: for a basic random walk
/// Transported: for an entity carried by another
#[derive(Clone, Copy, PartialEq)]
pub enum Behaviour {
    Playable,
    FreeWalk,
    Transported,
    Thrown {dir: Vec2, h: f32, thrower: u32},
}


/// For Playable behaviour
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
                    for other in entities.iter_mut() {
                        if other.id == id {
                        other.thrown(ent.direction, ent.id);
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

/// For FreeWalk behaviour
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

/// For Transportesd behaviour
fn transported(ent: &mut Entity, entities: &mut Vec<Entity>) {
    for other in entities.iter() {
        if other.id == ent.transporter.unwrap() {
            ent.direction = other.direction;
            //speed = other.max_speed;
            ent.direction = other.direction;                            // To keep the entity in the right direction
            ent.position.x = other.position.x;
            ent.position.y = other.position.y - other.collision_box.h;  // When transported, the entity is above

        }
    }
}
fn thrown(ent: &mut Entity, dir: Vec2, h: f32, thrower: u32) {
    ent.direction = dir;
    if dir.y == 0.0 {ent.direction.y = -0.5}
    else if dir.y * dir.y != 1.0 {ent.direction.y += 0.02}
    ent.apply_direction_with_speed(2.0);
    if h - 0.2 <= 0.0 {
        ent.behaviour = Behaviour::FreeWalk;

    } else {
        ent.behaviour = Behaviour::Thrown { dir: ent.direction, h: h - 0.2 , thrower};
    }

}
