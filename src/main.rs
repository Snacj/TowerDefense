extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    const ROWS: u32 = 12;
    const COLS: u32 = 16;

    const TILE_SIZE: u32 = 16;
    const SPRITE_SIZE: u32 = TILE_SIZE * 3;

    const WINDOW_WIDTH: u32 = SPRITE_SIZE * ROWS;
    const WINDOW_HEIGHT: u32 = SPRITE_SIZE * COLS;

    // print the window size
    println!("Window size: {} x {}", WINDOW_WIDTH, WINDOW_HEIGHT);

    let window = video_subsystem.window("Tower Defense", WINDOW_HEIGHT, WINDOW_WIDTH)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let tower = texture_creator.load_texture("assets/sprites/test.png")?;
    let man = texture_creator.load_texture("assets/sprites/man.png")?;
    let fluss = texture_creator.load_texture("assets/sprites/fluss2.png")?;
    let grass = texture_creator.load_texture("assets/sprites/grass.png")?;

    let tower1_dest_rect = Rect::new(10, 100, 64, 64);
    let mut man_dest_rect = Rect::new(300, 200, 64, 64);
    let fluss_dest_rect = Rect::new(0, 0, WINDOW_WIDTH * 2, WINDOW_HEIGHT);

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
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
        for i in 0..COLS {
            for j in 0..ROWS {
                let dest_rect = Rect::new((i * 16 * 3) as i32, (j * 16 * 3) as i32, 16*3, 16*3);
                canvas.copy(&grass, None, Some(dest_rect)).expect("Failed to copy grass texture");
            }
        }
        canvas.copy(&fluss, None, Some(fluss_dest_rect)).expect("Failed to copy tower texture");
        canvas.copy(&tower, None, Some(tower1_dest_rect)).expect("Failed to copy tower texture");
        canvas.copy(&man, None, Some(man_dest_rect)).expect("Failed to copy man texture");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
