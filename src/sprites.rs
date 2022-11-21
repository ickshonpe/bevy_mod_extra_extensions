use bevy::prelude::*;

pub trait ExtraSpriteFromExt {
    fn colored(color: Color) -> Self;
}

pub trait ExtraSpriteExt {
    fn with_color(self, color: Color) -> Self;
}

impl ExtraSpriteFromExt for Sprite {
    fn colored(color: Color) -> Self {
        Sprite {
            color,
            ..Default::default()
        }
    }
}

impl ExtraSpriteExt for Sprite {
    fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
