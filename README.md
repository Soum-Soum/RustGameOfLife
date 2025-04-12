# 🎮 Game of Life - Rust & Bevy 🦀

![Game of Life](https://img.shields.io/badge/Conway's-Game%20of%20Life-brightgreen)
![Rust](https://img.shields.io/badge/language-Rust-orange)
![Bevy](https://img.shields.io/badge/engine-Bevy-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 📜 Introduction

This project is an implementation of Conway's famous **"Game of Life"**, built with Rust and the Bevy game engine. It's my first Rust project, created to learn the basics of the language.

<p align="center">
  <video width="600" controls>
    <source src="./readme_resources/game_of_life_demo.mp4" type="video/mp4">
    Your browser does not support the video tag.
  </video>
</p>

## ✨ Features

- 🔲 Configurable grid size (currently 50x25)
- 🎨 Minimalist graphical interface with Bevy
- ⏱️ Simulation time control
- 🎲 Random cell generation
- 🖱️ Ability to place/remove cells with the mouse

## 🎯 Game Rules

The Game of Life follows simple but fascinating rules:

1. A living cell with fewer than 2 living neighbors dies (underpopulation)
2. A living cell with 2 or 3 living neighbors survives
3. A living cell with more than 3 living neighbors dies (overpopulation)
4. A dead cell with exactly 3 living neighbors becomes alive (reproduction)

## 🕹️ Controls

| Key | Action |
|--------|--------|
| `Space` | Pause/Resume simulation |
| `R` | Reset with random configuration |
| `C` | Clear all cells |
| `Left click` | Create/Delete a cell |

## 🔧 Installation

```bash
# Clone the repository
git clone https://github.com/your-name/game_of_life.git
cd game_of_life

# Build and run
RUST_BACKTRACE=1 cargo run --package game_of_life --bin game_of_life --features "bevy/dynamic_linking"
```

## 🏗️ Project Structure

```
src/
├── main.rs              # Main entry point
├── ui.rs                # User interface management
└── game_of_life/        # Game logic
    ├── clock.rs         # Time management
    ├── grid.rs          # Grid logic
    ├── mod.rs           # Modules
    └── systems.rs       # Bevy systems
```

## 📄 License

This project is under the MIT License. See the LICENSE file for more details.

---
