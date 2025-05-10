extern crate sdl2;
mod assets;
mod ui;
mod enemy;
mod tower;

use assets::Assets;
use enemy::Enemy;

use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::mouse::MouseButton;
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
    let mut enemy_dest_rect = Rect::new(64, 10, 64, 64);
    let background_dest_rect = Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    // define buttons
    // Button size
    let button_width = 64 * 4;
    let button_height = 32 * 4;

    // Total height of the two buttons plus spacing
    let total_buttons_height = button_height * 2;

    // X is centered based on button width
    let button_x = (WINDOW_WIDTH as i32 / 2 - button_width / 2) as i32;
    // Y is centered based on total height
    let button_y = (WINDOW_HEIGHT as i32 / 2 - total_buttons_height / 2) as i32;

    // Create buttons
    let start_game_button = ui::Button::new(button_x, button_y, button_width as u32, button_height as u32, &assets.start_button);
    let close_game_button = ui::Button::new(button_x, button_y + button_height, button_width as u32, button_height as u32, &assets.close_button);

    // Clear the canvas and update new renders
    canvas.clear();
    canvas.present();

    // Create event pump to handle events 
    let mut event_pump = sdl_context.event_pump().unwrap();

    // game state
    // 0 = Menu
    // 1 = Game
    #[derive(PartialEq)]
    enum GameState {
        Menu,
        Game,
    }

    pub struct GamePanel {
        pub game_state: GameState,
        pub tower_positions: Vec<Rect>, 
    }

    let mut game_panel = GamePanel {
        game_state: GameState::Menu,
        tower_positions: Vec::new(),
    };

    game_panel.game_state = GameState::Menu;

    // List of tower positions
    game_panel.tower_positions = Vec::new();

    // creating Enemy
    let mut enemy = Enemy::create_enemy();

    // Load the path as surface to check for collision
    // Converting to RGBA to check for transparency
    let mut path_surface = Surface::from_file("assets/sprites/fluss2.png")?;
    path_surface = path_surface.convert_format(PixelFormatEnum::RGBA8888)?;

    // Main game loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    game_panel.game_state = GameState::Menu;
                },

                // Track mouse coordinates on click
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    if game_panel.game_state == GameState::Menu {
                        // DEBUG
                        println!("mouse btn down at ({},{})", x, y);

                        // Check if the mouse is over the button
                        if start_game_button.is_hovered(x, y) {
                            println!("Button 1 clicked");
                            game_panel.game_state = GameState::Game;
                        } else if close_game_button.is_hovered(x, y) {
                            println!("Button 2 clicked");
                            break 'running;
                        }
                    } else {
                        if can_place_tower(&path_surface, x, y) {
                            game_panel.tower_positions.push(Rect::new(x - 32, y - 32, 64, 64));
                            println!("Tower placed at ({}, {})", x, y);
                        } else {
                            print!("Cannot place tower on path");
                        }
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
            game_panel.game_state = GameState::Game;
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

        // Render depending on the game state
        // Game State 1 = Game
        if game_panel.game_state == GameState::Game {

            if !enemy.finished {
                enemy.update();
                enemy_dest_rect.set_x(enemy.position.0 as i32);
                enemy_dest_rect.set_y(enemy.position.1 as i32);
            }        

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
            if !enemy.finished {
                canvas.copy(&assets.enemy, None, Some(enemy_dest_rect)).expect("Failed to copy enemy texture");
            }

            for tower in &game_panel.tower_positions {
                canvas.copy(&assets.tower, None, Some(*tower)).expect("Failed to copy tower texture");
            }

        // Game State 0 = Menu
        } else {
            canvas.copy(&assets.start_screen, None, Some(background_dest_rect)).expect("Failed to copy tower texture");
            start_game_button.draw(&mut canvas);
            close_game_button.draw(&mut canvas);
        }

        // Update the canvas
        canvas.present();

        // Delay to set the Framerate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 60 FPS
    }
    Ok(())
}

fn can_place_tower(surface: &Surface, x: i32, y: i32) -> bool {
    let tower_size = 64;
    let step = 8; // Check every 8 pixels to speed up, instead of every pixel

    for dx in (0..tower_size).step_by(step) {
        for dy in (0..tower_size).step_by(step) {
            let check_x = x + dx - tower_size / 2;
            let check_y = y + dy - tower_size / 2;

            if check_x < 0 || check_y < 0 {
                continue;
            }

            if is_occupied(surface, check_x as u32, check_y as u32) {
                return false;
            }
        }
    }

    true
}


fn is_occupied(surface: &Surface, xx: u32, yy: u32) -> bool {
    let x = xx / 4;
    let y = yy / 4;
    if x >= surface.width() || y >= surface.height() {
        return false; // click outside of surface
    }

    let pitch = surface.pitch() as usize;
    let bpp = surface.pixel_format_enum().byte_size_per_pixel();
    let surface_data = surface.without_lock().expect("Surface must not be locked");

    let index = (y as usize * pitch) + (x as usize * bpp);

    // Assume RGBA8888 format (little-endian: R G B A order)
    let a = surface_data[index + 3];

    a > 20 // threshold to avoid semi-transparent noise
}
