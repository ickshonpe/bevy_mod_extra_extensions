use bevy::prelude::*;

pub trait ColorWithExt {
    fn with_red(self, value: f32) -> Self;
    fn with_green(self, value: f32) -> Self;
    fn with_blue(self, value: f32) -> Self;
    fn with_hue(self, value: f32) -> Self;
    fn with_saturation(self, value: f32) -> Self;
    fn with_lightness(self, value: f32) -> Self;
    fn with_alpha(self, value: f32) -> Self;
}

impl ColorWithExt for Color {
    #[inline]
    #[must_use]
    fn with_red(mut self, r: f32) -> Self {
        self.set_r(r);
        self
    }

    #[inline]
    #[must_use]
    fn with_green(mut self, g: f32) -> Self {
        self.set_g(g);
        self
    }

    #[inline]
    #[must_use]
    fn with_blue(mut self, b: f32) -> Self {
        self.set_b(b);
        self
    }

    #[inline]
    #[must_use]
    fn with_alpha(mut self, a: f32) -> Self {
        self.set_a(a);
        self
    }

    #[inline]
    #[must_use]
    fn with_hue(mut self, hue: f32) -> Self {
        self = self.as_hsla();
        let Color::Hsla { hue: ref mut self_hue, .. } = self else { unreachable!() };
        *self_hue = hue;
        self
    }

    #[inline]
    #[must_use]
    fn with_saturation(mut self, saturation: f32) -> Self {
        self = self.as_hsla();
        let Color::Hsla { saturation: ref mut self_saturation, .. } = self else { unreachable!() };
        *self_saturation = saturation;
        self
    }

    #[inline]
    #[must_use]
    fn with_lightness(mut self, lightness: f32) -> Self {
        self = self.as_hsla();
        let Color::Hsla { saturation: ref mut self_lightness, .. } = self else { unreachable!() };
        *self_lightness = lightness;
        self
    }    
}