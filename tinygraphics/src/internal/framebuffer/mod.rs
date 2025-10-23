use core::{mem, ptr};

use embedded_graphics::{
    prelude::{Dimensions, PixelColor, RgbColor},
    primitives::Rectangle,
};
use libtinyos::syscalls::{self, PageTableFlags};

use crate::{
    GraphicsError,
    internal::abi::{BoundingBox, FRAMEBUFFER_START_ADDR, GFXConfig, KERNEL_FB, RawBitMap},
    utils::memset,
};

pub trait FrameBuffer {
    fn addr(&self) -> *mut u8;
    fn bpp(&self) -> u16;
    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn pitch(&self) -> u32;

    // offset in BYTES to self.addr(), where x and y are PIXEL coords
    fn pixel_offset(&self, x: u32, y: u32) -> u32;
    fn set_pixel<C: RgbColor>(&self, x: u32, y: u32, color: &C);
    fn fill<C: RgbColor>(&self, area: Rectangle, color: &C);
    fn flush(&self, bound: &BoundingBox) -> Result<(), GraphicsError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct FrameBufferDimensions {
    height: u32,
    width: u32,
    pitch: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameBufferConfig {
    pub red_mask_shift: u8,
    pub red_mask_size: u8,
    pub green_mask_shift: u8,
    pub green_mask_size: u8,
    pub blue_mask_shift: u8,
    pub blue_mask_size: u8,
    pub bpp: u16,
}

impl FrameBufferConfig {
    fn get_rgb_pixel<C: RgbColor>(&self, color: &C) -> u32 {
        let red = ((color.r() as u32) & ((1 << self.red_mask_size) - 1)) << self.red_mask_shift;
        let green =
            ((color.g() as u32) & ((1 << self.green_mask_size) - 1)) << self.green_mask_shift;
        let blue = ((color.b() as u32) & ((1 << self.blue_mask_size) - 1)) << self.blue_mask_shift;
        red | green | blue
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawFrameBuffer {
    buf: RawBitMap,
    dim: FrameBufferDimensions,
    config: FrameBufferConfig,
}

impl RawFrameBuffer {
    pub fn new() -> Self {
        let config = GFXConfig::new();
        let buf = unsafe { RawBitMap::new((config.height * config.pitch) as usize) };
        let dim = FrameBufferDimensions {
            height: config.height,
            width: config.width,
            pitch: config.pitch,
        };
        let config = FrameBufferConfig {
            red_mask_shift: config.red_mask_shift,
            red_mask_size: config.red_mask_size,
            green_mask_shift: config.green_mask_shift,
            green_mask_size: config.green_mask_size,
            blue_mask_shift: config.blue_mask_shift,
            blue_mask_size: config.blue_mask_size,
            bpp: config.bpp,
        };
        Self { buf, dim, config }
    }

    pub unsafe fn fill_row<C: RgbColor>(&self, x: u32, y: u32, len: u32, color: &C) {
        let offset = self.pixel_offset(x, y);
        let color = self.config.get_rgb_pixel(color);
        unsafe {
            memset(
                self.addr().add(offset as usize).cast::<u32>(),
                len as usize,
                color,
            )
        };
    }
}

impl FrameBuffer for RawFrameBuffer {
    fn addr(&self) -> *mut u8 {
        self.buf.addr()
    }

    fn bpp(&self) -> u16 {
        self.config.bpp
    }

    fn height(&self) -> u32 {
        self.dim.height
    }

    fn width(&self) -> u32 {
        self.dim.width
    }

    fn pitch(&self) -> u32 {
        self.dim.pitch
    }

    fn pixel_offset(&self, x: u32, y: u32) -> u32 {
        y * self.pitch() + x * (self.bpp() / 8) as u32
    }

    fn set_pixel<C: RgbColor>(&self, x: u32, y: u32, color: &C) {
        let offset = self.pixel_offset(x, y);
        let color = self.config.get_rgb_pixel(color);
        unsafe { self.addr().add(offset as usize).cast::<u32>().write(color) };
    }

    fn fill<C: RgbColor>(&self, area: Rectangle, color: &C) {
        let top_left = area.top_left.abs();
        for row in top_left.y as u32..top_left.y as u32 + area.size.height {
            unsafe {
                self.fill_row(top_left.x as u32, row, area.size.width, color);
            }
        }
    }

    fn flush(&self, _bound: &BoundingBox) -> Result<(), GraphicsError> {
        todo!(
            "RawBitmap needs to mmap the kernel fb somewhere and flush to it. (Only the case if this is not == kernel fb. (Maybe remove this for clarity))"
        )
    }
}

impl Default for RawFrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

// # SAFETY
// RawFramebuffer can only point to memory managed by the kernel.
// synchronization is also managed by the kernel
// the underlying memory will remanin valid for the entire lifetime of the program
unsafe impl Sync for RawFrameBuffer {}
unsafe impl Send for RawFrameBuffer {}

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub struct KernelFBWrapper {
    addr: *mut u8,
    dim: FrameBufferDimensions,
    config: FrameBufferConfig,
}

impl KernelFBWrapper {
    pub fn new() -> Self {
        let config = GFXConfig::new();

        let dim = FrameBufferDimensions {
            height: config.height,
            width: config.width,
            pitch: config.pitch,
        };
        let config = FrameBufferConfig {
            red_mask_shift: config.red_mask_shift,
            red_mask_size: config.red_mask_size,
            green_mask_shift: config.green_mask_shift,
            green_mask_size: config.green_mask_size,
            blue_mask_shift: config.blue_mask_shift,
            blue_mask_size: config.blue_mask_size,
            bpp: config.bpp,
        };

        let addr = FRAMEBUFFER_START_ADDR as *mut u8;

        let fb = unsafe {
            syscalls::open(
                KERNEL_FB.as_ptr(),
                KERNEL_FB.bytes().len(),
                syscalls::OpenOptions::WRITE,
            )
        }
        .unwrap();

        unsafe { syscalls::seek(fb, 0) }.unwrap();

        let size = (dim.pitch * dim.height) as usize;

        let addr = unsafe {
            syscalls::mmap(
                size,
                addr,
                PageTableFlags::USER_ACCESSIBLE
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::PRESENT,
                Some(fb),
            )
        }
        .unwrap();
        assert!(!addr.is_null());

        Self { addr, dim, config }
    }

    pub fn size(&self) -> usize {
        (self.dim.pitch * self.dim.height) as usize
    }

    pub unsafe fn fill_row<C: RgbColor>(&self, x: u32, y: u32, len: u32, color: &C) {
        let offset = self.pixel_offset(x, y);
        let color = self.config.get_rgb_pixel(color);
        unsafe {
            memset(
                self.addr().add(offset as usize).cast::<u32>(),
                len as usize,
                color,
            )
        };
    }
}

impl FrameBuffer for KernelFBWrapper {
    fn addr(&self) -> *mut u8 {
        self.addr
    }

    fn bpp(&self) -> u16 {
        self.config.bpp
    }

    fn height(&self) -> u32 {
        self.dim.height
    }

    fn width(&self) -> u32 {
        self.dim.width
    }

    fn pitch(&self) -> u32 {
        self.dim.pitch
    }

    fn pixel_offset(&self, x: u32, y: u32) -> u32 {
        y * self.pitch() + x * (self.bpp() / 8) as u32
    }

    fn set_pixel<C: RgbColor>(&self, x: u32, y: u32, color: &C) {
        let offset = self.pixel_offset(x, y);
        let color = self.config.get_rgb_pixel(color);
        unsafe { self.addr().add(offset as usize).cast::<u32>().write(color) };
    }

    fn fill<C: RgbColor>(&self, area: Rectangle, color: &C) {
        let top_left = area.top_left.abs();
        for row in top_left.y as u32..top_left.y as u32 + area.size.height {
            unsafe {
                self.fill_row(top_left.x as u32, row, area.size.width, color);
            }
        }
    }

    fn flush(&self, _bound: &BoundingBox) -> Result<(), GraphicsError> {
        // writes to this fb are immediately visible
        Ok(())
    }
}

// # SAFETY
// KernelFBWrapper can only point to memory managed by the kernel.
// synchronization is also managed by the kernel.
// however the kernel performs no synchronization.
// this is fine, as FB is write-only.
// the underlying memory will remanin valid for the entire lifetime of the program
unsafe impl Sync for KernelFBWrapper {}
unsafe impl Send for KernelFBWrapper {}
