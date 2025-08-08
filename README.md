# Bevy 2D Game Example

A simple 2D game built with the Bevy game engine, featuring a player sprite that can be controlled with keyboard input.

## Project Structure

```
├── assets/           # Game assets
│   ├── sprites/      # Sprite images
│   ├── sounds/       # Audio files
│   └── fonts/        # Font files
├── src/
│   ├── components/   # Game components
│   ├── systems/      # Game systems
│   ├── resources/    # Game resources
│   ├── plugins/      # Game plugins
│   ├── lib.rs        # Library code
│   └── main.rs       # Entry point
└── Cargo.toml        # Project configuration
```

## How to Run

```bash
cargo run
```

## Controls

- **WASD** or **Arrow Keys**: Move the player sprite
- **ESC**: Exit the game

## Features

- 2D sprite rendering
- Keyboard input handling
- Simple movement system
- Basic game loop with Bevy ECS

## Dependencies

- [Bevy](https://bevyengine.org/) - Game engine