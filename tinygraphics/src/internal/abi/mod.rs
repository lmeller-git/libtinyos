use core::{ptr::null_mut, str::FromStr};

use alloc::vec::{self, Vec};
use embedded_graphics::primitives::Rectangle;
use libtinyos::{
    println, syscall,
    syscalls::{self, OpenOptions, PageTableFlags},
};

use crate::{GraphicsError, internal::framebuffer::FrameBuffer};

pub const FRAMEBUFFER_START_ADDR: usize = 0x0000_3000_0000;

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
#[derive(PartialEq, Eq, Debug)]
pub struct RawBitMap {
    addr: *mut u8,
    size: usize,
}

impl RawBitMap {
    pub unsafe fn new(size: usize) -> Self {
        let addr = FRAMEBUFFER_START_ADDR as *mut u8;
        let addr = unsafe {
            syscalls::mmap(
                size,
                addr,
                PageTableFlags::WRITABLE
                    | PageTableFlags::PRESENT
                    | PageTableFlags::USER_ACCESSIBLE,
                None,
            )
        };

        let addr = addr.unwrap();
        assert!(!addr.is_null());

        Self { addr: addr, size }
    }

    pub fn new_from_kernel_fb(max_size: usize, offset: usize) -> Self {
        let addr = FRAMEBUFFER_START_ADDR as *mut u8;
        let fb = unsafe {
            syscalls::open(
                KERNEL_FB.as_ptr(),
                KERNEL_FB.bytes().len(),
                syscalls::OpenOptions::WRITE,
            )
        }
        .unwrap();

        unsafe { syscalls::seek(fb, offset) }.unwrap();

        let addr = unsafe {
            syscalls::mmap(
                max_size,
                addr,
                PageTableFlags::USER_ACCESSIBLE
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::PRESENT,
                Some(fb),
            )
        }
        .unwrap();
        assert!(!addr.is_null());

        Self {
            addr,
            size: max_size,
        }
    }

    pub fn addr(&self) -> *mut u8 {
        self.addr
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

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
        // TODO write abstraction for this in libtinyos::io
        let f = "/ram/.devconf/gfx/config.conf";
        let file =
            unsafe { syscalls::open(f.as_ptr(), f.bytes().len(), OpenOptions::READ) }.unwrap();
        let mut buffer = Vec::new();
        let mut idx = 0;
        buffer.extend_from_slice(&[0; 10]);
        while let Ok(read) =
            unsafe { syscalls::read(file, buffer[idx..].as_mut_ptr(), buffer.len() - idx, 0) }
            && read > 0
        {
            idx += read as usize;
            buffer.extend_from_slice(&[0; 10]);
        }

        let str_ = str::from_utf8(&buffer[..idx]).unwrap();
        let mut components = str_.split_whitespace();

        fn parse_t_from_str<T: FromStr>(
            components: &mut core::str::SplitWhitespace<'_>,
        ) -> Result<T, T::Err> {
            components.next().unwrap().parse()
        }

        Self {
            red_mask_shift: parse_t_from_str(&mut components).unwrap(),
            red_mask_size: parse_t_from_str(&mut components).unwrap(),
            green_mask_shift: parse_t_from_str(&mut components).unwrap(),
            green_mask_size: parse_t_from_str(&mut components).unwrap(),
            blue_mask_shift: parse_t_from_str(&mut components).unwrap(),
            blue_mask_size: parse_t_from_str(&mut components).unwrap(),
            bpp: parse_t_from_str(&mut components).unwrap(),
            width: parse_t_from_str(&mut components).unwrap(),
            height: parse_t_from_str(&mut components).unwrap(),
            pitch: parse_t_from_str(&mut components).unwrap(),
        }
    }
}

pub const KERNEL_FB: &str = "/proc/kernel/gfx/fb";

// pub fn raw_flush<F: FrameBuffer>(area: &BoundingBox, fb: &F) -> Result<(), GraphicsError> {
//     let fd = unsafe {
//         syscalls::open(
//             KERNEL_FB.as_ptr(),
//             KERNEL_FB.bytes().len(),
//             OpenOptions::WRITE,
//         )
//     }
//     .map_err(|_| GraphicsError::Unknown)?;
//     let dirty_area_start = unsafe {
//         fb.addr()
//             .add(fb.pixel_offset(area.x as u32, area.y as u32) as usize)
//     };
//     let dirty_area_end = unsafe {
//         fb.addr().add(
//             fb.pixel_offset((area.width + area.x) as u32, (area.height + area.y) as u32) as usize,
//         )
//     };

//     let dirty_area_length = unsafe { dirty_area_end.offset_from(dirty_area_start) };

//     println!(
//         "trying to write to fb a box of {:?}, at offset: {}, with len: {}",
//         area,
//         fb.pixel_offset(area.x as u32, area.y as u32) as usize,
//         dirty_area_length
//     );

//     unsafe { syscalls::seek(fd, fb.pixel_offset(area.x as u32, area.y as u32) as usize) }
//         .map_err(|_| GraphicsError::Unknown)?;
//     unsafe { syscalls::write(fd, dirty_area_start, dirty_area_length as usize) }
//         .map_err(|_| GraphicsError::Unknown)?;
//     Ok(())
// }
