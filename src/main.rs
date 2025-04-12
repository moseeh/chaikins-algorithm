mod app;

use ggez::event;
use ggez::winit::event_loop;
use ggez::{Context, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("CHAIKIN'S ALGORITHM", "MOSES")
        .build()
        .expect("Failed to build context");
    let state = app::AppState::new(&mut ctx);
    event::run(ctx, event_loop, state)
}
