mod app;

use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult {
    // ContextBuilder now uses a builder pattern
    let cb = ContextBuilder::new("chaikin", "moses")
        .window_setup(ggez::conf::WindowSetup::default().title("Chaikin's Algorithm"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    
    // Build context and event loop
    let (mut ctx, event_loop) = cb.build()?;
    
    // Create app state
    let state = app::AppState::new(&mut ctx);
    
    // Run with ownership of context and event_loop
    ggez::event::run(ctx, event_loop, state)
}