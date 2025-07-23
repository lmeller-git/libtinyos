use crate::{
    GraphicsError,
    internal::abi::{BoundingBox, raw_flush},
};
use alloc::vec::Vec;
use core::ops::{Add, AddAssign};
use embedded_graphics::prelude::Point;

pub use embedded_graphics::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct RGBColor(pub u8, pub u8, pub u8);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BatchedFlushes {
    inner: Vec<BoundingBox>,
}

impl BatchedFlushes {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn flush(&mut self) -> Result<(), GraphicsError> {
        raw_flush(&self.inner)?;
        self.inner.clear();
        Ok(())
    }

    pub fn push(&mut self, flush: BoundingBox) {
        self.inner.push(flush);
    }
}

impl Default for BatchedFlushes {
    fn default() -> Self {
        Self::new()
    }
}
