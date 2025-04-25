use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Button {
    pub rect: Rect,
    pub color: Color,
    pub label: &'static str,
}

impl Button {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: Color, label: &'static str) -> Self {
        Button {
            rect: Rect::new(x, y, w, h),
            color,
            label,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // TODO: instead of boring rectangle use nice texture 
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).unwrap();
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }
}

