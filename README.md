# Bevy Web Game

This is a minimal [Bevy](https://bevyengine.org) game set up to run on the web via WebAssembly and serve through GitHub Pages.

## Building

1. Install the `wasm32-unknown-unknown` target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Install [Trunk](https://trunkrs.dev):
   ```bash
   cargo install trunk
   ```
3. Build the project:
   ```bash
   trunk build --release
   ```
   The generated site will be output to the `docs` directory which can be served with GitHub Pages.

## Running locally

```bash
trunk serve --release
```

Then open <http://localhost:8080> in your browser to play the game.
