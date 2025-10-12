use core::ptr::null_mut;

use alloc::vec;
use embedded_graphics::primitives::Rectangle;
use libtinyos::{map_device, syscall};

use crate::GraphicsError;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BoundingBox {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl From<Rectangle> for BoundingBox {
    fn from(value: Rectangle) -> Self {
        assert!(value.top_left.x >= 0);
        assert!(value.top_left.y >= 0);

        Self {
            x: value.top_left.x as usize,
            y: value.top_left.y as usize,
            width: value.size.width as usize,
            height: value.size.height as usize,
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RawBitMap {
    addr: *mut u8,
    size: usize,
    gfx_fd: u32,
}

impl RawBitMap {
    pub unsafe fn new(size: usize) -> Self {
        let mut addr = null_mut();
        let fd = map_device(&mut addr);
        assert!(!addr.is_null());
        Self {
            addr: addr as *mut u8,
            size,
            gfx_fd: fd,
        }
    }

    pub fn addr(&self) -> *mut u8 {
        self.addr
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn fd(&self) -> u32 {
        self.gfx_fd
    }
}

#[deprecated]
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GFXConfig {
    pub red_mask_shift: u8,
    pub red_mask_size: u8,
    pub green_mask_shift: u8,
    pub green_mask_size: u8,
    pub blue_mask_shift: u8,
    pub blue_mask_size: u8,
    pub bpp: u16,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

impl GFXConfig {
    pub fn new() -> Self {
        let mut s = Self::default();

        _ = unsafe { syscall!(9, &mut s as *mut GFXConfig) };
        s
    }
}

pub fn raw_flush(bounding_boxes: &[BoundingBox], fd: u32) -> Result<(), GraphicsError> {
    let ptr = bounding_boxes.as_ptr();
    let len = bounding_boxes.len();
    let mut arr = vec![len];
    arr.extend_from_slice(unsafe {
        &*core::ptr::slice_from_raw_parts(ptr as *const usize, len * size_of::<BoundingBox>() / 64)
    });
    let ptr = arr.as_ptr();
    let res = libtinyos::write(fd as usize, ptr as *const u8, len);
    if res < 0 {
        Err(GraphicsError::default())
    } else {
        Ok(())
    }
}
