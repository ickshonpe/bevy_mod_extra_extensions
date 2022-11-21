use bevy::prelude::*;

pub trait ExtraTextStyleExt {
    fn with_font(self, font: Handle<Font>) -> Self;
    fn with_size(self, size: f32) -> Self;
    fn with_color(self, color: Color) -> Self;
}

impl ExtraTextStyleExt for TextStyle {
    #[inline]
    #[must_use]
    fn with_font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    #[inline]
    #[must_use]
    fn with_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    #[inline]
    #[must_use]
    fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
