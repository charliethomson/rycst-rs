

mod engine;
mod player;
mod vec2d;
mod config;
mod direction;

use engine::Engine;

fn main()  {
    let mut engine = Engine::new();

    engine.enter_loop();
}