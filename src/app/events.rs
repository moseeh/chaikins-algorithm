use crate::app::AppState;

pub fn handle_mouse_click(state: &mut AppState, x: f32, y: f32) {
    println!("Mouse clicked at: ({}, {})", x, y);
    state.control_points.push((x, y)); // adding clicked point to the vector of tuples
}
