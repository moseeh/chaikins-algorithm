use crate::chaikin::chaikin;
use crate::point::Point;
use macroquad::prelude::*;

pub async fn run() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut curve_steps: Vec<Vec<Point>> = Vec::new();
    let mut step = 0;
    let mut max_steps = 7;
    let mut animating = false;
    let mut last_step_time = get_time();
    let step_delay = 0.7;
    let mut dragging_index: Option<usize> = None;

    loop {
        clear_background(Color::from_rgba(204, 204, 204, 255)); // #CCCCCC

        let mouse_pos = Point {
            x: mouse_position().0,
            y: mouse_position().1,
        };

        if is_mouse_button_pressed(MouseButton::Left) && !animating {
            control_points.push(mouse_pos);
        }

        if is_mouse_button_pressed(MouseButton::Left)
            && !is_key_down(KeyCode::LeftShift)
            && !animating
        {
            for (i, point) in control_points.iter().enumerate() {
                if (point.x - mouse_pos.x).hypot(point.y - mouse_pos.y) < 10.0 {
                    dragging_index = Some(i);
                    break;
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(i) = dragging_index {
                control_points[i] = mouse_pos;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging_index = None;
        }

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

        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            curve_steps.clear();
            animating = false;
            step = 0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if animating && get_time() - last_step_time >= step_delay {
            if step < curve_steps.len() - 1 {
                step += 1;
                last_step_time = get_time();
            }
        }

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

        if control_points.len() == 1 {
            draw_circle(
                control_points[0].x + 2.0,
                control_points[0].y + 2.0,
                6.5,
                Color::new(0.0, 0.0, 0.0, 0.3),
            );
            draw_circle(control_points[0].x, control_points[0].y, 5.0, RED);
        } else if control_points.len() == 2 {
            draw_line(
                control_points[0].x,
                control_points[0].y,
                control_points[1].x,
                control_points[1].y,
                2.0,
                BLUE,
            );
        } else if animating && !curve_steps.is_empty() {
            let points = &curve_steps[step];
            for w in points.windows(2) {
                draw_line(
                    w[0].x + 2.0,
                    w[0].y + 2.0,
                    w[1].x + 2.0,
                    w[1].y + 2.0,
                    4.0,
                    Color::new(0.0, 0.0, 0.0, 0.2),
                );
                draw_line(
                    w[0].x,
                    w[0].y,
                    w[1].x,
                    w[1].y,
                    2.5,
                    Color::new(0.1, 0.2, 0.6, 0.85),
                );
            }
        } else {
            for w in control_points.windows(2) {
                draw_line(w[0].x, w[0].y, w[1].x, w[1].y, 1.0, GRAY);
            }
        }

        for p in &control_points {
            draw_circle(p.x + 2.0, p.y + 2.0, 6.5, Color::new(0.0, 0.0, 0.0, 0.3));
            draw_circle(p.x, p.y, 5.0, RED);
            draw_circle_lines(p.x, p.y, 7.0, 1.0, BLACK);
        }

        draw_text(
            "Click: Add Point | Drag: Move Point | Enter: Animate | C: Clear | Esc: Quit",
            20.0,
            30.0,
            20.0,
            BLACK,
        );
        draw_text(
            &format!("Max Steps: {}  |  Current Step: {}", max_steps, step + 1),
            20.0,
            55.0,
            20.0,
            BLACK,
        );

        next_frame().await;
    }
}
