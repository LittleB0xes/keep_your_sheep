use macroquad::texture::Texture2D;
use macroquad::math::Rect;

pub use hero::Hero;
mod hero;

pub use sheep::*;
mod sheep;

pub trait Entity {
    fn render(&mut self, texture: Texture2D, scale: f32);
    fn update(&mut self);
    fn get_collision_box(&self) -> Rect;
    fn get_y(&self) -> u32;
}

enum EntityType {
    Hero,
    Sheep
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum AnimationState {
    WalkSide,
    WalkUp,
    WalkDown
}

