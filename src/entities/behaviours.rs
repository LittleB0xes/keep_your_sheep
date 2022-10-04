use macroquad::math::Vec2;
use macroquad::input::*;
use macroquad::rand::gen_range;

use super::Entity;

#[derive(Clone)]
pub enum Behaviour {
    Playable,
    FreeWalk,
    Transported,
}


pub fn play_hero(hero: &mut Entity, entities: &Vec<Entity>) {
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
}