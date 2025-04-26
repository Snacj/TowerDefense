use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;

pub struct Assets<'a> {
    pub tower: Texture<'a>,
    pub man: Texture<'a>,
    pub fluss: Texture<'a>,
    pub grass: Texture<'a>,
    pub enemy: Texture<'a>,
    pub start_button: Texture<'a>,
    pub close_button: Texture<'a>,
    pub start_screen: Texture<'a>,
}

impl<'a> Assets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let tower = texture_creator.load_texture("assets/sprites/tower.png")?;
        let man = texture_creator.load_texture("assets/sprites/man.png")?;
        let fluss = texture_creator.load_texture("assets/sprites/fluss2.png")?;
        let grass = texture_creator.load_texture("assets/sprites/grass.png")?;
        let enemy = texture_creator.load_texture("assets/sprites/enemy.png")?;
        let start_button = texture_creator.load_texture("assets/sprites/start_game_button.png")?;
        let close_button = texture_creator.load_texture("assets/sprites/close_game_button.png")?;
        let start_screen = texture_creator.load_texture("assets/sprites/start_screen.png")?;

        Ok(Self {
            tower,
            man,
            fluss,
            grass,
            enemy,
            start_button,
            close_button,
            start_screen,
        })
    }
}

