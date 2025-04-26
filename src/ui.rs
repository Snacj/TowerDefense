use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Button<'a> {
    pub rect: Rect,
    pub texture: &'a Texture<'a>,
    pub label: &'static str,
}

impl<'a> Button<'a> {
    pub fn new(x: i32, y: i32, w: u32, h: u32, texture: &'a Texture<'a>, label: &'static str) -> Self {
        Button {
            rect: Rect::new(x, y, w, h),
            texture,
            label,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.copy(self.texture, None, Some(self.rect)).unwrap();
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }
}

