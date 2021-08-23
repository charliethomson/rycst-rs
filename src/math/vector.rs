use num_traits::AsPrimitive;
use sdl2::rect::Point;

use crate::state::{remap, remap_minz};

use super::wall::Wall;

#[derive(Debug, Clone, Copy)]
pub struct Vec2D {
    x: f64,
    y: f64,
    pub angle: f64,
    pub magnitude: f64,
}
impl Vec2D {
    pub const Origin: Self = Self {
        x: 0.,
        y: 0.,
        angle: 0.,
        magnitude: 0.,
    };

    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            angle: y.atan2(x),
            magnitude: x.hypot(y),
        }
    }

    pub fn from_angle(angle: f64) -> Self {
        Self::new(angle.cos(), angle.sin()).normalize()
    }

    pub fn apply<F: FnOnce((f64, f64)) -> (f64, f64)>(&self, f: F) -> Self {
        let (x, y) = f(self.x_y());
        Self::new(x, y)
    }

    pub fn normalize(self) -> Self {
        Self::new(self.x / self.magnitude, self.y / self.magnitude)
    }

    pub fn dist(&self, other: &Self) -> f64 {
        (*self + -*other).magnitude
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0. && self.y == 0.
    }

    pub fn intersects(&self, wall: &Wall) -> Option<Vec2D> {
        let (x1, y1) = wall.a.x_y();
        let (x2, y2) = wall.b.x_y();
        let (x3, y3) = self.x_y();
        let x4: f64 = self.x() + self.angle.cos();
        let y4: f64 = self.y() + self.angle.sin();

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if den == 0.0 {
            return None;
        }

        let num1 = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let num2 = -1.0 * ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3));

        let t = num1 / den;
        let u = num2 / den;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            Some(Self::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
        } else {
            None
        }
    }

    pub fn clamp(&mut self, dims: (u32, u32), padding: f64) {
        if self.x < padding {
            self.set_x(padding)
        }

        if self.x > dims.0 as f64 - padding {
            self.set_x(dims.0 as f64 - padding)
        }

        if self.y < padding {
            self.set_y(padding)
        }

        if self.y > dims.1 as f64 - padding {
            self.set_y(dims.1 as f64 - padding)
        }
    }

    pub fn x_y(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
        self.angle = self.y.atan2(x);
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
        self.angle = self.x.atan2(y);
    }

    pub fn set_x_raw(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y_raw(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
        self.x += self.angle.cos();
        self.y += self.angle.sin();
    }
    pub fn set_angle_raw(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn add_x_y_raw(&mut self, x_y: (f64, f64)) {
        let (x, y) = x_y;
        self.x += x;
        self.y += y;
    }

    pub fn translate(&mut self, other: &Self) -> Self {
        self.x += other.x();
        self.y += other.y();

        *self
    }

    pub fn remap<OD: Into<Self>, ND: Into<Self>>(&self, old_dims: OD, new_dims: ND) -> Self {
        let old_vector: Self = old_dims.into();
        let new_vector: Self = new_dims.into();
        Self::new(
            remap_minz(self.x, old_vector.x, new_vector.x),
            remap_minz(self.y, old_vector.y, new_vector.y),
        )
    }
}
impl From<Vec2D> for Point {
    fn from(this: Vec2D) -> Self {
        Point::new(this.x.round() as i32, this.y.round() as i32)
    }
}
impl<F: AsPrimitive<f64>> From<(F, F)> for Vec2D {
    fn from((x, y): (F, F)) -> Self {
        Self::new(x.as_(), y.as_())
    }
}
impl std::ops::Mul<f64> for Vec2D {
    type Output = Vec2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl std::ops::MulAssign<f64> for Vec2D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}
impl std::ops::Add for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl std::ops::AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::Neg for Vec2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}
