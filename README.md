# Be The Cashier

`Be The Cashier` is a small top-down pixel-art game prototype built in Rust with `macroquad`.

At the moment, the project includes a controllable cashier character, animated movement, a horizontally extended tilemap, camera follow with shake, and collision against wall tiles.

## Stack

- Rust
- macroquad
- serde
- serde_json

## Run

Make sure you have Rust installed, then run:

```bash
cargo run
```

To check the project without launching the game window:

```bash
cargo check
```

## Controls

- `W`: move up
- `A`: move left
- `S`: move down
- `D`: move right

## Current Features

- Player sprite animation for idle and running states
- Horizontal expansion of the original tilemap layout
- Camera follow with subtle movement shake
- Tile-based wall collision
- Pixel-art rendering with nearest-neighbor filtering

## Project Structure

- [src/main.rs](/home/jake/Documentos/BeTheCashier/src/main.rs): game loop, fixed timestep, camera, and rendering flow
- [src/host.rs](/home/jake/Documentos/BeTheCashier/src/host.rs): player movement, animation, collision response, and drawing
- [src/world.rs](/home/jake/Documentos/BeTheCashier/src/world.rs): tilemap generation, solid-tile rules, and world rendering
- [src/config.rs](/home/jake/Documentos/BeTheCashier/src/config.rs): window, player, camera, and tile constants
- [map/tileset.png](/home/jake/Documentos/BeTheCashier/map/tileset.png): tileset image
- [map/tileset-data.json](/home/jake/Documentos/BeTheCashier/map/tileset-data.json): tile metadata
- [sprites/host.png](/home/jake/Documentos/BeTheCashier/sprites/host.png): player sprite sheet

## Notes

- The map currently stretches only horizontally.
- Some wall collision rules are defined directly in code based on tile IDs used by the current map.
- This is still a prototype, so the game loop and content are intentionally simple.
