# CHAIKIN

Chaikin is an interactive drawing tool built with Macroquad that visualizes the Chaikin corner-cutting algorithm.  
Users can define control points on a 2D canvas and see a smooth curve generated through iterative refinement.  
Animations reveal the curve evolving step-by-step, making it ideal for both educational and creative purposes.  
Control points can be added with Shift+Click and repositioned by dragging.  
Step depth is adjustable in real time, providing fine control over curve smoothness.

## Features

- Add, drag, and clear control points with intuitive mouse and keyboard input
- Run Chaikin's algorithm with configurable iteration depth
- Real-time step-by-step animation of curve evolution
- Clean UI with point highlighting and instructions overlay
- Adjustable refinement level with keyboard shortcuts

## Controls

| Action                  | Key / Mouse                      |
|-------------------------|----------------------------------|
| Add control point       | Shift + Left Click               |
| Drag point              | Left Click + Drag               |
| Start animation         | Enter                            |
| Clear all               | C                                |
| Increase steps          | = or L                           |
| Decrease steps          | - or K                           |
| Exit                    | Escape                           |

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Add Macroquad:  
   In your `Cargo.toml`, include:

   ```toml
   [dependencies]
   macroquad = "0.3"
   ```

## Run the project:

```bash
    cargo run --release
```

## Screenshots

//TODO

## Authors
