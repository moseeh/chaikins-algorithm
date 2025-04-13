mod app;

use ggez::{ContextBuilder, GameResult, event};

fn main() -> GameResult {
    // ContextBuilder gives us (Context, EventLoop)
    let (mut ctx, mut event_loop) = ContextBuilder::new("CHAIKIN'S ALGORITHM", "MOSES").build()?; // Use ? for error propagation

    // Create app state
    let mut state = app::AppState::new(&mut ctx);

    // Run with mutable references to context and event_loop
    event::run(&mut ctx, &mut event_loop, &mut state)
}
