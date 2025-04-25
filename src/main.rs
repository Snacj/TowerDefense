extern crate sdl2;
mod assets;
mod ui;

use assets::Assets;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Window Size in Rows and Columns
    const ROWS: u32 = 12;
    const COLS: u32 = 16;

    // Initial Sprite Size in px
    const TILE_SIZE: u32 = 16;
    // Scaled Sprite Size in px
    const SPRITE_SIZE: u32 = TILE_SIZE * 4; // 64x64 px

    // Window Size in px
    const WINDOW_WIDTH: u32 = SPRITE_SIZE * COLS;
    const WINDOW_HEIGHT: u32 = SPRITE_SIZE * ROWS;

    // Create a window with { Title, Width, Height }
    let window = video_subsystem.window("Tower Defense", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas to draw on
    let mut canvas = window.into_canvas().build().unwrap();

    // Create a texture context
    let texture_creator = canvas.texture_creator();

    // Load textures
    let assets = Assets::load(&texture_creator).unwrap();

    // Set destination rectangles for the textures to set size and position
    let tower1_dest_rect = Rect::new(64, 10, 64, 64);
    let mut man_dest_rect = Rect::new(300, 200, 64, 64);
    let fluss_dest_rect = Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    // define buttons
    let button1 = ui::Button::new(10, 10, 100, 50, sdl2::pixels::Color::RGB(0, 255, 0), "Button 1");
    let button2 = ui::Button::new(120, 10, 100, 50, sdl2::pixels::Color::RGB(255, 0, 0), "Button 2");

    // Clear the canvas and update new renders
    canvas.clear();
    canvas.present();

    // Create event pump to handle events 
    let mut event_pump = sdl_context.event_pump().unwrap();

    // game state
    let mut game_state = 0;

    // Main game loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    
                    break 'running
                },

                // Track mouse coordinates on click
                Event::MouseButtonDown { x, y, .. } => {
                    println!("mouse btn down at ({},{})", x, y);
                    if button1.is_hovered(x, y) {
                        println!("Button 1 clicked");
                        game_state = 1;
                    } else if button2.is_hovered(x, y) {
                        println!("Button 2 clicked");
                        break 'running;
                    }
                },
                _ => {}
            }
        }

        // Get the current state of the keyboard
        let keyboard_state = event_pump.keyboard_state();

        // Man movement
        // Remember that the origin (0,0) is at the top left corner of the window so Y increases
        // downwards
        // W = Up
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            man_dest_rect.set_y(man_dest_rect.y() - 2);
            game_state = 1;
        }
        // S = Down
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            man_dest_rect.set_y(man_dest_rect.y() + 2);
        }
        // A = Left
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            man_dest_rect.set_x(man_dest_rect.x() - 2);
        }
        // D = Right
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            man_dest_rect.set_x(man_dest_rect.x() + 2);
        }

        if game_state == 1 {

            // Render grass tiles on every Row Col
            for i in 0..COLS {
                for j in 0..ROWS {
                    let dest_rect = Rect::new((i * SPRITE_SIZE) as i32, (j * SPRITE_SIZE) as i32, SPRITE_SIZE, SPRITE_SIZE);
                    canvas.copy(&assets.grass, None, Some(dest_rect)).expect("Failed to copy grass texture");
                }
            }
            // Render rest of the textures in Order
            canvas.copy(&assets.fluss, None, Some(fluss_dest_rect)).expect("Failed to copy tower texture");
            canvas.copy(&assets.tower, None, Some(tower1_dest_rect)).expect("Failed to copy tower texture");
            canvas.copy(&assets.man, None, Some(man_dest_rect)).expect("Failed to copy man texture");
        } else {
            button1.draw(&mut canvas);
            button2.draw(&mut canvas);
        }

        // Update the canvas
        canvas.present();

        // Delay to set the Framerate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 60 FPS
    }
    Ok(())
}
