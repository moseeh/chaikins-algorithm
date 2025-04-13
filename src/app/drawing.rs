use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, MeshBuilder, DrawParam, Mesh};

pub fn draw_canvas(
    ctx: &mut Context,
    control_points: &[(f32, f32)],
    canvas: &mut Canvas,
) -> GameResult {
    let mut mb = MeshBuilder::new();
    
    for &(x, y) in control_points.iter() {
        mb.circle(
            graphics::DrawMode::fill(),
            [x, y],
            5.0,
            0.1,
            Color::BLACK,
        )?;
    }
    
    let mesh = Mesh::from_data(ctx, mb.build());
    canvas.draw(&mesh, DrawParam::default());
    
    Ok(())
}