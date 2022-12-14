use macroquad::prelude::*;

use crate::sprite_library::SpriteLibraryData;

#[derive(Clone, Copy)]
pub struct Sprite {
    position: Vec2,
    pub source_rect: Rect,
    speed: i32,
    frames: i32,
    elapsed: i32,
    current_frame: i32,
    pub flip_x: bool,
    play: bool,
}

impl Sprite {
    pub fn new(data: SpriteLibraryData) -> Self {
        let source_rect = Rect::new(data.x as f32, data.y as f32, data.w as f32, data.h as f32);
        Self {
            position: Vec2::ZERO,
            source_rect,
            frames: data.frame,
            speed: data.speed,
            elapsed: 0,
            current_frame: 0,
            flip_x: false,
            play: true,
        }
    }

    fn animate(&mut self) {
        self.elapsed += 1;
        if self.elapsed > self.speed {
            self.current_frame = (self.current_frame + 1) % self.frames;
            self.elapsed = 0;
        }
    }

    pub fn draw_sprite(&mut self, texture: Texture2D, scale: f32) {
        if self.play {
            self.animate();
        }
        let current_source_rect = Rect {
            x: self.source_rect.x + self.source_rect.w * self.current_frame as f32,
            y: self.source_rect.y,
            w: self.source_rect.w,
            h: self.source_rect.h,
        };
        let params = DrawTextureParams {
            source: Some(current_source_rect),
            dest_size: Some(Vec2::new(self.source_rect.w * scale, self.source_rect.h * scale)),
            rotation: 0.0,
            flip_x: self.flip_x,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            texture,
            (self.position.x * scale).round(),
            (self.position.y * scale).round(),
            WHITE,
            params,
        );
    }

    pub fn set_animation(&mut self, data: &SpriteLibraryData) {
        self.source_rect = Rect::new(data.x as f32, data.y as f32, data.w as f32, data.h as f32);
        self.frames = data.frame;
        self.speed = data.speed;
    }

    pub fn set_position_to(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn play(&mut self) {
        self.play = true;
        self.current_frame = 0;
    }
    //pub fn stop(&mut self) {
    //    self.play = false;
    //    self.current_frame = 0;
    //}
}
