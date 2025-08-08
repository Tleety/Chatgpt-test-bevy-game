# Development Instructions

This document provides instructions for developing and running the Bevy 2D game.

## Prerequisites

- Rust (latest stable version)
- Graphics drivers (for running the full game with window)

## Building and Running

### Build the project
```bash
cargo build
```

### Run the full game (requires display)
```bash
cargo run
```
Note: This will open a window with the game. Use WASD or arrow keys to move the blue square.

### Run tests
```bash
cargo test
```

### Run headless simulation (no display required)
```bash
cargo run --example headless
```
This demonstrates that the game systems work correctly without requiring a display.

## Development Commands

### Check for compilation errors
```bash
cargo check
```

### Run with optimizations (release mode)
```bash
cargo run --release
```

### Build examples
```bash
cargo build --examples
```

## Project Structure

- `src/main.rs` - Main game entry point with window setup
- `src/lib.rs` - Library code and tests
- `src/components/` - Game components (Player, Velocity)
- `src/systems/` - Game systems (movement, input)
- `src/plugins/` - Game plugins that bundle systems
- `src/resources/` - Game resources (empty but ready for expansion)
- `examples/` - Example projects demonstrating functionality
- `assets/` - Game assets (sprites, sounds, fonts)

## Adding New Features

1. **New Components**: Add to `src/components/mod.rs`
2. **New Systems**: Add to `src/systems/` and register in plugin
3. **New Assets**: Place in appropriate `assets/` subdirectory
4. **New Examples**: Create in `examples/` directory

## Troubleshooting

### "Failed to build event loop" error
This occurs when running the main game without a display. Use the headless example instead:
```bash
cargo run --example headless
```

### Missing dependencies
Make sure all system dependencies are installed:
- On Ubuntu/Debian: `sudo apt install build-essential pkg-config`
- On other systems, consult the Bevy documentation