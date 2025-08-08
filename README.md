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

### Desktop
```bash
cargo run
```

### Web (GitHub Pages)
The game is automatically deployed to GitHub Pages when changes are pushed to the main branch. You can play it at:
`https://tleety.github.io/Chatgpt-test-bevy-game/`

### Local Web Development
Install [Trunk](https://trunkrs.dev/) and run:
```bash
trunk serve
```
Then open http://localhost:8080 in your browser.

## Controls

- **WASD** or **Arrow Keys**: Move the player sprite
- **ESC**: Exit the game

## Features

- 2D sprite rendering
- Keyboard input handling
- Simple movement system
- Basic game loop with Bevy ECS
- **Web deployment** - Playable in any modern web browser via GitHub Pages
- Cross-platform support (Desktop and Web)

## Dependencies

- [Bevy](https://bevyengine.org/) - Game engine