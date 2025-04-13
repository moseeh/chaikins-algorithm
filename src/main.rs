
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

fn chaikin(points: &[Point]) -> Vec<Point> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();
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

#[macroquad::main("Chaikin Animation")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut curve_steps: Vec<Vec<Point>> = Vec::new();
    let mut step = 0;
    let mut animating = false;
    let mut last_step_time = get_time();
    let step_delay = 0.7;
    let mut dragging_index: Option<usize> = None;

    loop {
        clear_background(WHITE);

        let mouse_pos = Point {
            x: mouse_position().0,
            y: mouse_position().1,
        };

        if is_mouse_button_pressed(MouseButton::Left) && !animating {
            control_points.push(mouse_pos);
        }

        if is_key_pressed(KeyCode::Enter) {
            if control_points.len() > 1 {
                curve_steps.clear();
                let mut current = control_points.clone();
                for _ in 0..7 {
                    current = chaikin(&current);
                    curve_steps.push(current.clone());
                }
                animating = true;
                step = 0;
                last_step_time = get_time();
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) && !animating {
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

        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            curve_steps.clear();
            step = 0;
            animating = false;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if animating && get_time() - last_step_time >= step_delay {
            step = (step + 1) % 7;
            last_step_time = get_time();
        }

        if control_points.len() == 1 {
            draw_circle(control_points[0].x + 2.0, control_points[0].y + 2.0, 6.5, Color::new(0.0, 0.0, 0.0, 0.3));
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

        next_frame().await;
    }
}