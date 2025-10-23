use alloc::boxed::Box;
use core::marker::PhantomData;
use embedded_graphics::{
    Drawable,
    prelude::{Dimensions, DrawTarget, OriginDimensions, Point, Primitive, RgbColor},
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
};
use libtinyos::{
    println,
    syscalls::{self, FileDescriptor, OpenOptions},
};

use crate::{GraphicsError, backend::GraphicsBackend, internal::abi::BoundingBox};

pub use crate::internal::framebuffer::{FrameBuffer, KernelFBWrapper, RawFrameBuffer};

pub struct PrimitiveDrawer<'a, B, C>
where
    B: FrameBuffer,
    C: RgbColor,
{
    buf: &'a B,
    _phantom: PhantomData<C>,
}

impl<'a, B, C> PrimitiveDrawer<'a, B, C>
where
    B: FrameBuffer,
    C: RgbColor,
{
    pub fn new(buf: &'a B) -> Self {
        Self {
            buf,
            _phantom: PhantomData,
        }
    }
}

// impl<C> Default for PrimitiveDrawer<'_, RawFrameBuffer, C>
// where
//     C: RgbColor,
// {
//     fn default() -> Self {
//         // this will leak, however the drawer should be alive for the entire lifetime anyways
//         let buf = Box::leak(Box::new(RawFrameBuffer::new()));
//         Self::new(buf)
//     }
// }

impl<C> Default for PrimitiveDrawer<'_, KernelFBWrapper, C>
where
    C: RgbColor,
{
    fn default() -> Self {
        // this will leak, however the drawer should be alive for the entire lifetime anyways
        let buf = Box::leak(Box::new(KernelFBWrapper::new()));
        Self::new(buf)
    }
}

impl<B, C> DrawTarget for PrimitiveDrawer<'_, B, C>
where
    B: FrameBuffer,
    C: RgbColor,
{
    type Color = C;
    type Error = GraphicsError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.buf
                .set_pixel(pixel.0.x as u32, pixel.0.y as u32, &pixel.1);
        }
        Ok(())
    }
}

impl<B, C> OriginDimensions for PrimitiveDrawer<'_, B, C>
where
    B: FrameBuffer,
    C: RgbColor,
{
    fn size(&self) -> embedded_graphics::prelude::Size {
        embedded_graphics::prelude::Size::new(self.buf.width(), self.buf.height())
    }
}

impl<B, C> GraphicsBackend for PrimitiveDrawer<'_, B, C>
where
    B: FrameBuffer,
    C: RgbColor,
{
    type Color = C;
    fn flush(&mut self, bounds: Rectangle) -> Result<(), GraphicsError> {
        self.buf.flush(&bounds.into())
    }

    fn draw_primitive<D>(&mut self, glyph: &D) -> Result<D::Output, GraphicsError>
    where
        D: Drawable<Color = Self::Color> + Dimensions,
    {
        glyph.draw(self)
    }
}
