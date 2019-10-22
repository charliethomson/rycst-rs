

pub struct Vec2D {
    pub x: f64,
    pub y: f64,
    // Radians
    pub angle: f64,
    pub magnitude: f64,
} impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D {
            x,
            y,
            angle: 0.0,
            magnitude: 1.0,
        }
    }

    pub fn new_all(x: f64, y: f64, angle: f64, magnitude: f64) -> Self {
        Vec2D {
            x,
            y,
            angle,
            magnitude,
        }
    }

    pub fn opposite(&self) -> Self {
        let nx = self.x + (self.magnitude * self.angle.sin());
        let ny = self.y + (self.magnitude * self.angle.cos());
        let na = self.angle + std::f64::consts::PI;
        Self::new_all(nx, ny, na, self.magnitude)
    }

}