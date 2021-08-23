use crate::math::vector::Vec2D;

pub struct Wall {
    pub a: Vec2D,
    pub b: Vec2D,
    pub color_index: usize,
}
impl Wall {
    pub fn new(a: Vec2D, b: Vec2D, color_index: usize) -> Self {
        Self { a, b, color_index }
    }
}
