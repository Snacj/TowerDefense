# 🏰 Rusty Defense

_A 2D pixel-art tower defense game built with Rust and SDL2._

---

## 📦 Tech Stack

- **Language**: Rust
- **Graphics**: SDL2 (`rust-sdl2` crate)
- **Build System**: Cargo
- **Platform**: Cross-platform (Windows, Linux, macOS)

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- SDL2 library installed on your system:
  - **macOS**: `brew install sdl2`
  - **Ubuntu/Debian**: `sudo apt-get install libsdl2-dev`
  - **Windows**: Use [vcpkg](https://github.com/microsoft/vcpkg) or install SDL2 manually

### Clone and Run

```bash
git clone https://github.com/yourusername/rusty-defense.git
cd rusty-defense
cargo run
```

---

## 🧱 Project Structure (Planning)

```
src/
├── main.rs           # Entry point
├── game.rs           # Game state and loop
├── tower.rs          # Tower behavior and upgrades
├── enemy.rs          # Enemy pathfinding and logic
├── level.rs          # Level layout and wave management
├── ui.rs             # UI components like health bar, score, etc.
├── utils.rs          # Utility functions
assets/
├── sprites/          # Towers, enemies, background tiles
├── sounds/           # Background music and SFX
```

---

## 🎮 Features (Planned)

- [x] Game loop with SDL2 window
- [x] Tile-based map rendering
- [ ] Placeable towers
- [ ] Enemy waves with pathfinding
- [ ] Tower attack logic (range, damage, fire rate)
- [ ] Upgrade system for towers
- [ ] UI: health, gold, wave indicator
- [ ] Sound effects and background music
- [ ] Win/lose conditions

---

## 🗺️ Gameplay Concept

- **Objective**: Defend your base by strategically placing towers that attack incoming enemies.
- **Resources**: Earn gold by defeating enemies and use it to upgrade towers or place new ones.
- **Progression**: Enemies arrive in waves with increasing difficulty.

---

## 🎨 Assets

- **Sprites**: Placeholder pixel art from [Kenney.nl](https://kenney.nl/assets) or custom
- **Audio**: Chiptune background music and effects (public domain or CC0)

---

## 📅 Roadmap

| Feature                  | Status  | Notes                                  |
|--------------------------|---------|----------------------------------------|
| Basic tower placement    | 🔲 TODO | Drag & drop or click-to-place system   |
| Enemy wave system        | 🔲 TODO | Spawn based on wave intervals          |
| Pathfinding              | 🔲 TODO | A* or fixed path for enemies           |
| Collision & range logic  | 🔲 TODO | Check if enemy is in tower's radius    |
| Tower upgrades           | 🔲 TODO | Increase range, damage, fire rate      |
| HUD elements             | 🔲 TODO | Health, wave number, resources         |
| Sound integration        | 🔲 TODO | Background music and attack SFX        |

---

## 🧠 Inspirations

- **Games**: *Kingdom Rush*, *Bloons TD*, *Plants vs. Zombies*
- **Style**: Retro pixel graphics
- **Music**: Chiptune arcade vibe

---

## 🧪 Tests

> To be implemented alongside core mechanics

---

## 🙌 Contributing

Pull requests are welcome! Please open an issue first for any big changes or suggestions.

---

## 📜 License

MIT License
