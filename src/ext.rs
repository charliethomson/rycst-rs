use sdl2::pixels::Color;

pub(crate) trait ColorExt {
    fn dim(&self, by: u8) -> Self;
}

impl ColorExt for Color {
    fn dim(&self, by: u8) -> Self {
        Self::RGB(
            self.r.saturating_sub(by),
            self.g.saturating_sub(by),
            self.b.saturating_sub(by),
        )
    }
}
