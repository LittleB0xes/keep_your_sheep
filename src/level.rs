use std::path::Path;
use std::fs::{File, self};

use macroquad::texture::{Texture2D, draw_texture_ex, load_image};
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};

use macroquad::texture::DrawTextureParams;
use serde::Deserialize;


#[derive(Deserialize)]
struct SimplifiedLdtk{
    width: i32,
    height: i32,
    layers: Vec<String>,
}


pub struct Level  {
    pub cell_w: usize,
    pub cell_h: usize,
    pub width: f32,
    pub height: f32,
    pub collision_grid: Vec<u8>,
}


impl Level {
    pub fn new() -> Level {
        let data_path = Path::new(&"./assets/sheep/simplified/Level_0/data.json");
        let data_file = File::open(data_path).expect("erreur de lecture - ldtk");
        let data: SimplifiedLdtk = serde_json::from_reader(data_file).unwrap();

        let collision_raw = fs::read_to_string("./assets/sheep/simplified/Level_0/Collision.csv").expect("erreur lecture cvs file");
        let collision_grid = extract_cvs(collision_raw);

        Level {
            cell_w: (data.width / 16) as usize,
            cell_h: (data.height / 16) as usize,
            width: data.width as f32,
            height: data.height as f32,
            collision_grid,
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



/// A basic approch, just for a specific use
fn extract_cvs(raw_data: String) -> Vec<u8> {
    let mut output = Vec::new();
    for i in 0..raw_data.len() {
        if raw_data.chars().nth(i).unwrap() as u8 <= 57 && raw_data.chars().nth(i).unwrap() as u8 >= 48 {
            output.push(raw_data.chars().nth(i).unwrap() as u8 - 48);
        }
    }
    output
}