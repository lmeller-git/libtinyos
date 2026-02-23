use core::{slice, str::Utf8Error};

use crate::internal::rt::runtime;

// TODO
// these should really return their own iterator types
// migth also want some lifetime data?

pub fn env<'a>() -> Option<&'a EnvVars> {
    runtime().env()
}

pub fn args<'a>() -> Option<&'a ProcessArgs> {
    runtime().args()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvErr {
    Utf8(Utf8Error),
    InvalidPtr,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ProcessArgs {
    at: *const u8,
    len: usize,
}

impl ProcessArgs {
    /// creates a new ProcessArgs instance and validates teh data pointed to by at.
    /// returns an err if the ptr is invalid, or the data does not contain a valid str
    pub fn new(at: *const u8, len: usize) -> Result<Self, EnvErr> {
        if at.is_null() {
            return Err(EnvErr::InvalidPtr);
        }
        let _sl = unsafe { slice::from_raw_parts(at, len) };
        let _s = str::from_utf8(_sl).map_err(EnvErr::Utf8)?;
        Ok(Self { at, len })
    }

    /// creates a new ProcessArgs instance without performing any validation.
    pub unsafe fn new_unchecked(at: *const u8, len: usize) -> Self {
        Self { at, len }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.at, self.len) }
    }

    pub fn as_split_bytes(&self) -> impl Iterator<Item = &[u8]> {
        self.as_bytes().split(|b| *b == b' ')
    }

    pub fn as_split_str(&self) -> impl Iterator<Item = &str> {
        self.as_split_bytes()
            .map(|bytes| unsafe { str::from_utf8_unchecked(bytes) })
    }

    pub fn named(&self, name: &str) -> Option<&str> {
        let name = name.as_bytes();
        self.as_split_bytes()
            .find(|slice| *slice == name)
            .map(|bytes| unsafe { str::from_utf8_unchecked(bytes) })
    }

    pub fn nth(&self, n: usize) -> Option<&str> {
        self.as_split_str().nth(n)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct EnvVars {
    at: *const u8,
    len: usize,
}

impl EnvVars {
    /// creates a new EnvVars instance and validates teh data pointed to by at.
    /// returns an err if the ptr is invalid, or the data does not contain a valid str
    pub fn new(at: *const u8, len: usize) -> Result<Self, EnvErr> {
        if at.is_null() {
            return Err(EnvErr::InvalidPtr);
        }
        let _sl = unsafe { slice::from_raw_parts(at, len) };
        let _s = str::from_utf8(_sl).map_err(EnvErr::Utf8)?;
        Ok(Self { at, len })
    }

    /// creates a new EnvVars instance without performing any validation.
    pub unsafe fn new_unchecked(at: *const u8, len: usize) -> Self {
        Self { at, len }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.at, self.len) }
    }

    pub fn as_split_bytes(&self) -> impl Iterator<Item = (&[u8], &[u8])> {
        self.as_bytes().split(|b| *b == b'\0').map(|entry| {
            let mut split = entry.splitn(2, |b| *b == b'=');
            (
                split.next().unwrap_or_default(),
                split.next().unwrap_or_default(),
            )
        })
    }

    pub fn as_split_str(&self) -> impl Iterator<Item = (&str, &str)> {
        self.as_split_bytes().map(|(k, v)| {
            (unsafe { str::from_utf8_unchecked(k) }, unsafe {
                str::from_utf8_unchecked(v)
            })
        })
    }

    pub fn separate_byte_entries<'a>(
        it: impl Iterator<Item = (&'a [u8], &'a [u8])>,
    ) -> impl Iterator<Item = (&'a [u8], impl Iterator<Item = &'a [u8]>)> {
        it.map(|(k, v)| (k, v.split(|b| *b == b';')))
    }

    pub fn separate_str_entries<'a>(
        it: impl Iterator<Item = (&'a str, &'a str)>,
    ) -> impl Iterator<Item = (&'a str, impl Iterator<Item = &'a str>)> {
        it.map(|(k, v)| (k, v.split(';')))
    }

    pub fn get(&self, var: &str) -> Option<&str> {
        let var = var.as_bytes();
        self.as_split_bytes()
            .find(|(k, _v)| *k == var)
            .map(|(_k, v)| unsafe { str::from_utf8_unchecked(v) })
    }
}
