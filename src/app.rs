pub mod app_tests;
use crate::chaikin::chaikin;
use crate::point::Point;
use macroquad::prelude::*;

fn generate_curve_steps(control_points: &[Point], max_steps: usize) -> Vec<Vec<Point>> {
    let mut steps = Vec::new();
    if control_points.len() > 1 {
        let mut current = control_points.to_vec();
        for _ in 0..max_steps {
            current = chaikin(&current);
            steps.push(current.clone());
        }
    }
    steps
}

pub async fn run() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut curve_steps: Vec<Vec<Point>> = Vec::new();
    let mut step = 0;
    let mut max_steps = 7;
    let mut animating = false;
    let mut last_step_time = get_time();
    let step_delay = 0.5;
    let mut dragging_index: Option<usize> = None;
    let mut clicked_enter = false;

    loop {
        clear_background(Color::from_rgba(204, 204, 204, 255));

        let mouse_pos = Point {
            x: mouse_position().0,
            y: mouse_position().1,
        };

        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut near_existing = false;

            // Check if clicking near existing point
            for (i, point) in control_points.iter().enumerate() {
                if (point.x - mouse_pos.x).hypot(point.y - mouse_pos.y) < 10.0 {
                    dragging_index = Some(i);
                    near_existing = true;
                    break;
                }
            }

            // Add new point only when not animating and curve not generated
            if !near_existing && !animating && curve_steps.is_empty() {
                control_points.push(mouse_pos);
            }
        }

        // Update dragged point and regenerate curve if needed
        if let Some(i) = dragging_index {
            control_points[i] = mouse_pos;
            if !curve_steps.is_empty() {
                curve_steps = generate_curve_steps(&control_points, max_steps);
                if animating {
                    step = 0;
                    last_step_time = get_time();
                } else {
                    step = curve_steps.len().saturating_sub(1);
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging_index = None;
        }

        // Keyboard controls
        if is_key_pressed(KeyCode::Enter) && control_points.len() > 1 {
            curve_steps = generate_curve_steps(&control_points, max_steps);
            animating = true;
            clicked_enter = false;
            step = 0;
            last_step_time = get_time();
        } else if is_key_pressed(KeyCode::Enter) && control_points.len() < 1 {
            clicked_enter = true ;
        }

        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            curve_steps.clear();
            animating = false;
            step = 0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Adjust steps
        if is_key_pressed(KeyCode::Minus) && max_steps > 1 {
            max_steps -= 1;
            if !curve_steps.is_empty() {
                curve_steps = generate_curve_steps(&control_points, max_steps);
                step = step.min(curve_steps.len().saturating_sub(1));
            }
        }

        if is_key_pressed(KeyCode::Equal) && max_steps < 10 {
            max_steps += 1;
            if !curve_steps.is_empty() {
                curve_steps = generate_curve_steps(&control_points, max_steps);
                step = step.min(curve_steps.len().saturating_sub(1));
            }
        }

        // Animation update
        if animating && get_time() - last_step_time >= step_delay {
            if step < curve_steps.len().saturating_sub(1) {
                step += 1;
                last_step_time = get_time();
            } else {
                animating = false;
            }
        }

        // Drawing
        // Draw control lines
        for w in control_points.windows(2) {
            draw_line(w[0].x, w[0].y, w[1].x, w[1].y, 1.0, GRAY);
        }

        // Draw curve if generated
        if !curve_steps.is_empty() {
            let points = &curve_steps[step];
            for w in points.windows(2) {
                // Shadow effect
                draw_line(
                    w[0].x + 2.0,
                    w[0].y + 2.0,
                    w[1].x + 2.0,
                    w[1].y + 2.0,
                    4.0,
                    Color::new(0.0, 0.0, 0.0, 0.2),
                );
                // Main curve
                draw_line(
                    w[0].x,
                    w[0].y,
                    w[1].x,
                    w[1].y,
                    2.5,
                    Color::new(0.1, 0.2, 0.6, 0.85),
                );
            }
        }

        // Draw control points
        for p in &control_points {
            let hovered = (p.x - mouse_pos.x).hypot(p.y - mouse_pos.y) < 10.0;
            let color = if hovered { YELLOW } else { RED };
            
            draw_circle(p.x + 2.0, p.y + 2.0, 6.5, Color::new(0.0, 0.0, 0.0, 0.3));
            draw_circle(p.x, p.y, 5.0, color);
            draw_circle_lines(p.x, p.y, 7.0, 1.0, BLACK);
        }

        // UI text
        draw_text(
            "Click: Add Point | Drag: Move Point | Enter: Start Animation | +/-: Steps | C: Clear | Esc: Quit",
            20.0,
            30.0,
            20.0,
            BLACK,
        );
        draw_text(
            &format!("Steps: {}/{}  |  Points: {}", step + 1, max_steps, control_points.len()),
            20.0,
            55.0,
            20.0,
            BLACK,
        );

        // Empty state message
        if control_points.is_empty() {
            if clicked_enter {
                draw_text(
                    "Click anywhere to add control points then Press Enter",
                    screen_width() / 3.0 - 160.0,
                    screen_height() / 2.0,
                    30.0,
                    DARKGRAY,
                );
            } else {
                draw_text(
                    "Click anywhere to add control points",
                    screen_width() / 2.0 - 140.0,
                    screen_height() / 2.0,
                    30.0,
                    DARKGRAY,
                );
            }
        }

        next_frame().await;
    }
}