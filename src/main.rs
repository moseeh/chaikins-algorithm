mod point;
mod chaikin;
mod app;

#[macroquad::main("Chaikin Animation")]
async fn main() {
    app::run().await;
}
