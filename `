use macroquad::prelude::*;

struct Game {
    texture: Texture2D,
}

impl Game {
    fn new() -> Self {
        let texture = Texture2D::from_file_with_format(include_bytes!("../assets/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);

        Self {
            texture,

        }
    }

    fn update(&mut self) {}

    fn render(&mut self) {}
}


#[macroquad::main(window_conf())]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();

        game.render();
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RuneLighter".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        //high_dpi: true,
        ..Default::default()
    }
}
