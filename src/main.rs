

mod map;
mod input;
mod config;
mod engine;
mod render;
mod player;

use coffee::graphics::WindowSettings;
use coffee::Game;
use coffee::Result;
use crate::config::{ WIDTH, HEIGHT };

fn main() -> Result<()> {
    engine::Engine::run(WindowSettings {
        title: "Raycast dot are ess :)".to_owned(),
        size: (WIDTH, HEIGHT),
        resizable: false,
        fullscreen: false,
    })
}
