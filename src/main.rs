extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Tower Defense", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let tower = texture_creator.load_texture("assets/sprites/test.png")?;
    let man = texture_creator.load_texture("assets/sprites/man.png")?;

    let tower1_dest_rect = Rect::new(100, 100, 64, 64);
    let mut man_dest_rect = Rect::new(300, 200, 64, 64);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            man_dest_rect.set_y(man_dest_rect.y() - 5);
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            man_dest_rect.set_y(man_dest_rect.y() + 5);
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            man_dest_rect.set_x(man_dest_rect.x() - 5);
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            man_dest_rect.set_x(man_dest_rect.x() + 5);
        }

        // The rest of the game loop goes here...
        canvas.copy(&tower, None, Some(tower1_dest_rect)).unwrap();
        canvas.copy(&man, None, Some(man_dest_rect)).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
