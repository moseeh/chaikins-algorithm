// Ensure you import the necessary items
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};

mod drawing;

pub struct AppState {
    pub control_points: Vec<(f32, f32)>,
    pub animating: bool,
    // Remove the persistent canvas; we don't need to store it between frames.
    // canvas: Canvas,
}

impl AppState {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            control_points: Vec::new(),
            animating: false,
            // No longer need a canvas in state.
            // canvas: Canvas::from_frame(ctx, Color::WHITE),
        }
    }
}

impl EventHandler for AppState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new canvas for the current frame with the desired clear color.
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        // Draw to the canvas (pass the canvas as mutable reference)
        drawing::draw_canvas(ctx, &self.control_points, &mut canvas)?;

        // Finish the canvas to render the frame
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.control_points.push((x, y));
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Return) => {
                if self.control_points.len() >= 2 {
                    self.animating = true;
                } else {
                    println!("Add at least 2 points first");
                }
            }
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => {}
        }
        Ok(())
    }
}
