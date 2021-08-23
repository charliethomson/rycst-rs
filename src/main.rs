mod app;
mod ext;
mod key_state_handler;
mod map;
mod math;
mod state;
use app::App;

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

fn main() -> Result<(), String> {
    App::new()?.start()?;

    Ok(())
}
