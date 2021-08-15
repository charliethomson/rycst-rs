mod app;
mod map;
mod state;
use app::App;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    App::new()?.start()?;

    Ok(())
}
