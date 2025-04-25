use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;

pub struct Assets<'a> {
    pub tower: Texture<'a>,
    pub man: Texture<'a>,
    pub fluss: Texture<'a>,
    pub grass: Texture<'a>,
    pub enemy: Texture<'a>,
}

impl<'a> Assets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let tower = texture_creator.load_texture("assets/sprites/test.png")?;
        let man = texture_creator.load_texture("assets/sprites/man.png")?;
        let fluss = texture_creator.load_texture("assets/sprites/fluss2.png")?;
        let grass = texture_creator.load_texture("assets/sprites/grass.png")?;
        let enemy = texture_creator.load_texture("assets/sprites/enemy.png")?;

        Ok(Self {
            tower,
            man,
            fluss,
            grass,
            enemy,
        })
    }
}

