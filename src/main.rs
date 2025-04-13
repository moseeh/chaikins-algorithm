use macroquad::prelude::*;

// Basic 2D point struct used for both control and generated points
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

// Chaikin's corner-cutting algorithm: smooths a polyline by interpolating new points
fn chaikin(points: &[Point]) -> Vec<Point> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::with_capacity((points.len() - 1) * 2);
    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];

        let q = Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        };
        let r = Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        };

        new_points.push(q);
        new_points.push(r);
    }
    new_points
}

// Main event loop and drawing logic
#[macroquad::main("Chaikin Animation")]
async fn main() {
    // Stores original control points set by user
    let mut control_points: Vec<Point> = Vec::new();

    // Stores all intermediate steps from Chaikin algorithm
    let mut curve_steps: Vec<Vec<Point>> = Vec::new();

    // Current animation step index
    let mut step = 0;

    // Maximum number of Chaikin iterations to compute
    let mut max_steps = 7;

    // Whether animation is running
    let mut animating = false;

    // For time-based step progression
    let mut last_step_time = get_time();
    let step_delay = 0.7;

    // Optional index of the point being dragged
    let mut dragging_index: Option<usize> = None;

    loop {
        // Clear screen each frame
        clear_background(WHITE);

        // Get current mouse position as Point
        let mouse_pos = Point {
            x: mouse_position().0,
            y: mouse_position().1,
        };

        // Add point if Shift+Click is used and not animating
        if is_key_down(KeyCode::LeftShift) && is_mouse_button_pressed(MouseButton::Left) && !animating {
            control_points.push(mouse_pos);
        }

        // Begin dragging if mouse click is near an existing point
        if is_mouse_button_pressed(MouseButton::Left) && !is_key_down(KeyCode::LeftShift) && !animating {
            for (i, point) in control_points.iter().enumerate() {
                if (point.x - mouse_pos.x).hypot(point.y - mouse_pos.y) < 10.0 {
                    dragging_index = Some(i);
                    break;
                }
            }
        }

        // Update dragging point to follow mouse
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(i) = dragging_index {
                control_points[i] = mouse_pos;
            }
        }

        // Stop dragging on mouse release
        if is_mouse_button_released(MouseButton::Left) {
            dragging_index = None;
        }

        // Begin animation when Enter is pressed and points are valid
        if is_key_pressed(KeyCode::Enter) && control_points.len() > 1 {
            curve_steps.clear();
            let mut current = control_points.clone();
            for _ in 0..max_steps {
                current = chaikin(&current);
                curve_steps.push(current.clone());
            }
            animating = true;
            step = 0;
            last_step_time = get_time();
        }

        // Clear all points and animation with C
        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            curve_steps.clear();
            animating = false;
            step = 0;
        }

        // Quit program on Escape
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Step through animation based on time
        if animating && get_time() - last_step_time >= step_delay {
            if step < curve_steps.len() - 1 {
                step += 1;
                last_step_time = get_time();
            }
        }

        // Adjust max number of steps dynamically
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::K) {
            if max_steps > 1 {
                max_steps -= 1;
            }
        }

        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::L) {
            if max_steps < 10 {
                max_steps += 1;
            }
        }

        // ===== Drawing Section =====

        // One point: show dot only
        if control_points.len() == 1 {
            draw_circle(control_points[0].x + 2.0, control_points[0].y + 2.0, 6.5, Color::new(0.0, 0.0, 0.0, 0.3));
            draw_circle(control_points[0].x, control_points[0].y, 5.0, RED);

        // Two points: show a line
        } else if control_points.len() == 2 {
            draw_line(
                control_points[0].x,
                control_points[0].y,
                control_points[1].x,
                control_points[1].y,
                2.0,
                BLUE,
            );

        // Animated curve: draw current Chaikin step
        } else if animating && !curve_steps.is_empty() {
            let points = &curve_steps[step];
            for w in points.windows(2) {
                draw_line(
                    w[0].x + 2.0, w[0].y + 2.0,
                    w[1].x + 2.0, w[1].y + 2.0,
                    4.0,
                    Color::new(0.0, 0.0, 0.0, 0.2),
                );
                draw_line(
                    w[0].x, w[0].y,
                    w[1].x, w[1].y,
                    2.5,
                    Color::new(0.1, 0.2, 0.6, 0.85),
                );
            }

        // Fallback: show raw control lines
        } else {
            for w in control_points.windows(2) {
                draw_line(w[0].x, w[0].y, w[1].x, w[1].y, 1.0, GRAY);
            }
        }

        // Draw all control point markers
        for p in &control_points {
            draw_circle(p.x + 2.0, p.y + 2.0, 6.5, Color::new(0.0, 0.0, 0.0, 0.3));
            draw_circle(p.x, p.y, 5.0, RED);
            draw_circle_lines(p.x, p.y, 7.0, 1.0, BLACK);
        }

        // UI overlay with instructions and step info
        draw_text("Shift+Click: Add Point | Drag: Move Point | Enter: Animate | C: Clear | Esc: Quit", 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(&format!("Max Steps: {}  |  Current Step: {}", max_steps, step + 1), 20.0, 55.0, 20.0, DARKGRAY);
        draw_text("[-]/K: Decrease Steps | [+]/L: Increase Steps", 20.0, 80.0, 20.0, DARKGRAY);

        // Advance to next frame
        next_frame().await;
    }
}
