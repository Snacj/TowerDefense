use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Button<'a> {
    pub name: String,
    pub rect: Rect,
    pub texture: &'a Texture<'a>,
}

impl<'a> Button<'a> {
    pub fn new(name: String, x: i32, y: i32, w: u32, h: u32, texture: &'a Texture<'a>) -> Self {
        Button {
            name,
            rect: Rect::new(x, y, w, h),
            texture,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.copy(self.texture, None, Some(self.rect)).unwrap();
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }

    pub fn handle_button_click(&self) {
        // Handle different button clicks
        match self.name.as_str() {
            "Start" => println!("Start button clicked!"),
            "Exit" => println!("Exit button clicked!"),
            _ => println!("Unknown button clicked!"),
        }
    }
}

