use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder};
use ggez::{Context, GameResult};

pub fn draw_canvas(ctx: &mut Context, control_points: &[(f32, f32)]) -> GameResult {
    graphics::clear(ctx, Color::from_rgb(255, 255, 255));
    let mut mb = MeshBuilder::new();
    for &(x, y) in control_points.iter() {
        mb.circle(
            DrawMode::Fill(()),
            [x, y],
            5.0,
            0.1,
            Color::from_rgb(0, 0, 0),
        )?;
    }
    let mesh = mb.build()?;
    graphics::draw(ctx, &mesh, DrawParam::default())?;
    graphics::present(ctx)
}
