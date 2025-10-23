mod expose;
pub(crate) mod raw;

use crate::GraphicsError;
use embedded_graphics::{
    Drawable,
    mono_font::{self, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::{Dimensions, Point, Primitive, RgbColor},
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Styled, StyledDrawable,
    },
    text::Text,
};
pub use expose::*;

pub trait GraphicsBackend {
    type Color: RgbColor;
    fn flush(&mut self, bounds: Rectangle) -> Result<(), GraphicsError>;

    fn draw_primitive<D>(&mut self, glyph: &D) -> Result<D::Output, GraphicsError>
    where
        D: Drawable<Color = Self::Color> + Dimensions;
}
