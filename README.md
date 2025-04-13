# CHAIKIN

Chaikin is an interactive drawing tool built with Macroquad that visualizes the Chaikin corner-cutting algorithm.  
Users can define control points on a 2D canvas and see a smooth curve generated through iterative refinement.  
Animations reveal the curve evolving step-by-step, making it ideal for both educational and creative purposes.  
Control points can be added with Left Click and repositioned by dragging.  
Step depth is adjustable in real time, providing fine control over curve smoothness.

## Features

- Add, drag, and clear control points with intuitive mouse input
- Run Chaikin's algorithm with configurable iteration depth (1-10 steps)
- Real-time step-by-step animation of curve evolution
- Interactive point manipulation after curve generation
- Clean UI with point highlighting and instructional overlay
- Adjustable refinement level with keyboard shortcuts
- Shadow effects for better visual depth
- Responsive design with empty state guidance

## Controls

| Action                  | Key / Mouse                      |
|-------------------------|----------------------------------|
| Add control point       | Left Click                       |
| Drag point              | Left Click + Drag                |
| Start animation         | Enter                            |
| Clear all               | C                                |
| Increase steps (max 10) | = or L                           |
| Decrease steps (min 1)  | - or K                           |
| Exit                    | Escape                           |

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Add Macroquad as a dependency in your `Cargo.toml`:

```toml
[dependencies]
macroquad = "0.4"
```

## Running the Project

```bash
cargo run --release
```

## Usage Workflow

1. **Add Points**: Click anywhere on the canvas to create control points
2. **Generate Curve**: Press Enter to start the subdivision animation
3. **Modify Live**: Drag points during/after animation to update the curve
4. **Adjust Smoothness**: Use +/- keys to change subdivision depth
5. **Reset**: Press C to clear the canvas and start fresh


## Authors
1. [Moses onyango](https://learn.zone01kisumu.ke/git/moonyango)
2. [Hezbon Shikuku](https://learn.zone01kisumu.ke/git/hshikuku)
3. [jesee kuya](https://learn.zone01kisumu.ke/git/jkuya)

---

*Note: Requires Rust 1.54+ and Macroquad 0.4+. Tested on Windows/Linux/MacOS.*