use macroquad::input::*;
use macroquad::math::{Vec2, Rect};
use macroquad::rand::gen_range;

use crate::entities::Entity;
use crate::level::{self, Level};

/// the main puppet_master's function
pub fn play(entities: &mut Vec<Entity>, level: &Level) {
    // Apply each entity's behaviours
    for i in 0..entities.len() {
        let mut ent = entities[i].clone();
        match ent.behaviour {
            Behaviour::Playable => playable(&mut ent, entities),
            Behaviour::FreeWalk => free_walk(&mut ent),
            Behaviour::Transported => transported(&mut ent, entities),
            Behaviour::Thrown { dir, yo, h, thrower } => thrown(&mut ent, dir, yo, h, thrower),
        }
        // Replace by the new updated entity
        entities[i] = ent;
    }

    // Check collision between entities
    entity_entity_collision(entities, level);

    // Make all entities move
    motion(entities);
}

/// Check collision between entities
pub fn entity_entity_collision(entities: &mut Vec<Entity>, level: &Level) {
    // Collision detection


    // detection on x and y to allow collide and slide
    for i in 0..entities.len() {
        let mut ent = entities[i].clone();

        // First, we need to stay in the playground
        let collider = ent.get_collision_box();
        if collider.x < 0.0 || collider.x + collider.w > level.width {
            ent.direction.x = 0.0;
            ent.velocity.x = 0.0; 
        }
        if collider.y < 0.0 || collider.y + collider.h > level.height {
            ent.direction.y = 0.0;
            ent.velocity.y = 0.0; 
        }


        for j in 0..entities.len() {
            // Avoid collison if transported or with a transported thing
            let avoid_collision = match (ent.behaviour, entities[j].behaviour) {
                (_, Behaviour::Transported) | (Behaviour::Transported, _) => true,
                _ => false,
            };

            let collision =ent.collidable && !avoid_collision && entities[j].collidable && ent.id != entities[j].id;

            // On x
            if collision
                && ent
                    .get_collision_box_diff(true, false)
                    .overlaps(&entities[j].get_collision_box())
            {
                ent.direction.x = 0.0;
                ent.velocity.x = 0.0;
            }

            // on y
            if collision
                && ent
                    .get_collision_box_diff(false, true)
                    .overlaps(&entities[j].get_collision_box())
            {
                ent.direction.y = 0.0;
                ent.velocity.y = 0.0;
            }
        }
        // Collision, background collision grid... a basic one
        for i  in 0..level.collision_grid.len() {

            
            let cell_x = ((i % level.cell_w) * 16) as f32;
            let cell_y = ((i / level.cell_w) * 16) as f32;
            // on x
            if  level.collision_grid[i] != 0 && ent.get_collision_box_diff(true, false).overlaps(&Rect::new(cell_x, cell_y + 6.0, 16.0, 10.0)) {
                ent.direction.x = 0.0;
                ent.velocity.x = 0.0; 
            }

            // on y
            if  level.collision_grid[i] != 0 && ent.get_collision_box_diff(false, true).overlaps(&Rect::new(cell_x, cell_y + 6.0, 16.0, 10.0)) {
                ent.direction.y = 0.0;
                ent.velocity.y = 0.0;
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
    Thrown { dir: Vec2, yo: f32, h: f32, thrower: u32},
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
                        other.thrown(ent.direction, ent.position.y, ent.id);
                        ent.drop();
                    }
                }
            }
            None => {
                for other in entities.iter_mut() {
                    let dist = (ent.get_collision_box().center()
                        - other.get_collision_box().center())
                    .length_squared();
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
            // To keep the entity in the right direction
            ent.direction = other.direction;

            // When transported, the entity is above
            ent.position = Vec2::new(other.position.x, other.position.y - other.collision_box.h);
        }
    }
}
fn thrown(ent: &mut Entity, dir: Vec2, yo: f32, h: f32, thrower: u32) {
    ent.direction = dir;
    if dir.y == 0.0 {
        ent.direction.y = -0.5
    } else if dir.y * dir.y != 1.0 {
        ent.direction.y += 0.02
    }
    ent.apply_direction_with_speed(2.0);
    if h - 0.2 <= 0.0 {
        ent.behaviour = Behaviour::FreeWalk;
        ent.collidable = true;
    } else {
        ent.behaviour = Behaviour::Thrown {
            dir: ent.direction,
            yo: yo,
            h: h - 0.2,
            thrower,
        };
        ent.collidable = false;
    }
}