pub mod drawing;
pub mod events; // handle events (mouse and keyboard inputs) // handle drawing on the canvas

use ggez::event::EventHandler;
use ggez::{Context, GameResult};

pub struct AppState {
    pub control_points: Vec<(f32, f32)>,
    pub animating: bool,
}

impl AppState {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            control_points: Vec::new(),
            animating: false,
        }
    }
}

impl EventHandler for AppState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // for now no update logic.
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        drawing::draw_canvas(ctx, &self.control_points)
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == ggez::event::MouseButton::Left {
            events::handle_mouse_click(self, x, y)
        }
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _mods: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            ggez::event::KeyCode::Return => {
                if self.control_points.len() >= 2 {
                    self.animating = true
                } else {
                    println!("Please add at least 2 control points first.");
                }
            }
            ggez::event::KeyCode::Escape => ggez::event::quit(ctx),
            _ => {}
        }
    }
}