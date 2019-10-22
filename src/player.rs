
use crate::{
    config::{
        P_RD,
        P_IP,
        P_MS,
    },
    direction::Direction,
    vec2d::Vec2D
};

pub struct Player {
    position: Vec2D,
} impl Player {
    pub fn new() -> Self {
        Player {
            position: Vec2D::new_all(
                P_IP.0,
                P_IP.1,
                0.0,
                P_RD
            )
        }
    }

    pub fn mv(&mut self, dir: Direction) {
        let angle = match dir {
            Direction::Forward => {
                self.position.angle
            },
            Direction::Backward => {
                self.position.angle + std::f64::consts::PI
            },
            Direction::Left => {
                self.position.angle + std::f64::consts::FRAC_PI_2
            },
            Direction::Right => {
                self.position.angle - std::f64::consts::FRAC_PI_2
            },
        };
        let offset: (f64, f64) = (
            self.position.x + (P_MS * angle.cos()),
            self.position.y + (P_MS * angle.sin())
            );

        self.position.x += offset.0;
        self.position.y += offset.1;
    }

    pub fn draw(&mut self) {}
}