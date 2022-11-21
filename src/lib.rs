pub mod text_style;
pub mod color;
pub mod sprites;

pub mod prelude {
    pub use crate::text_style::ExtraTextStyleExt;
    pub use crate::color::ExtraColorExt;
    pub use crate::sprites::ExtraSpriteFromExt;
}