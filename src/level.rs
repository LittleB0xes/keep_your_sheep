use std::path::Path;
use std::fs::File;

use macroquad::texture::{Texture2D, draw_texture_ex};
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};

use macroquad::texture::DrawTextureParams;
use serde::Deserialize;

use crate::entities::Entity;

#[derive(Deserialize)]
struct SimplifiedLdtk {
    //pub x: i32,
    //pub y: i32,
    pub width: i32,
    pub height: i32,
    //pub iid: String,
    pub layers: Vec<String>,
}


pub struct Level {
    width: f32,
    height: f32,
    colision_grid: Vec<u8>,
    objects: Vec<Entity>
}

impl Level {
    pub fn new(level_name: String) -> Level {
        let ldtk_root = "./assets/sheep/simplified".to_string();
        let path = format!("{}/{}/data.json", ldtk_root, level_name);
        let data_path = Path::new(&path);
        //let data_path = Path::new("./assets/sheep/simplified/Level_0/data.json");
        let data_file = File::open(data_path).expect("erreur de lecture - ldtk");
        let data: SimplifiedLdtk = serde_json::from_reader(data_file).unwrap();

        
        Level {
            width: data.width as f32,
            height: data.height as f32,
            colision_grid: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn render(&mut self, texture: Texture2D, scale: f32) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.width * scale, self.height * scale)),
            source: Some(Rect::new(0.0, 0.0, self.width, self.height)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None };
        draw_texture_ex(texture, 0.0, 0.0, WHITE, params);
    }


}