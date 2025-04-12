pub mod events; // handle events (mouse and keyboard inputs)
pub mod drawing; // handle drawing on the canvas

use ggez::{Context, GameResult};
use ggez::event::EventHandler;

pub struct AppState {
    pub control_points : Vec<(f32,f32)>,
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