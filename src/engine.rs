
use winit::{

};
use winit_window::WinitWindow;
use piston::window::{self, WindowSettings};

use crate::{
    player::Player,
    vec2d::Vec2D,
};

pub struct Engine {
    window: WinitWindow,
    player: Player,

} impl Engine {
    pub fn new() -> Self {
        let player = Player::new();
        let mut settings = WindowSettings::new("TEST WINDOW", (800, 600)).exit_on_esc(true);
        settings.set_automatic_close(true);
        settings.set_resizable(false); //.build().expect("Failed to create window"),
        let window = WinitWindow::new(
            &settings
        );
        Engine {
            window,
            player
        }
    }

    pub fn enter_loop(self) {
        loop {

        }
    }
}