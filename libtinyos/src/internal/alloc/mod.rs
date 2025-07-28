pub use ::alloc::*;

use crate::request_heap;

use conquer_once::spin::OnceCell;
use core::{
    alloc::GlobalAlloc,
    ptr::{NonNull, null_mut},
};
use linked_list_allocator::LockedHeap;

const START_HEAP_SIZE: usize = 1024 * 500;
const HEAP_EXT: usize = 1024 * 100;

#[global_allocator]
pub static GLOBAL_ALLOC: EnsureInitAlloc = EnsureInitAlloc::empty();

pub struct EnsureInitAlloc {
    inner: OnceCell<LockedHeap>,
}

impl EnsureInitAlloc {
    const fn empty() -> Self {
        Self {
            inner: OnceCell::uninit(),
        }
    }

    fn init(&self) {
        _ = self.inner.try_init_once(|| {
            let heap = request_heap(START_HEAP_SIZE);
            if heap.is_null() {
                panic!("Allocator could not be initialized");
            }
            let locked_heap = LockedHeap::empty();
            unsafe { locked_heap.lock().init(heap, START_HEAP_SIZE) };
            locked_heap
        });
    }

    fn request(&self) -> Result<(), ()> {
        let ptr = request_heap(HEAP_EXT);
        if ptr.is_null() {
            return Err(());
        }
        unsafe { self.inner.get().unwrap().lock().extend(HEAP_EXT) };
        Ok(())
    }
}

unsafe impl GlobalAlloc for EnsureInitAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if !self.inner.is_initialized() {
            self.init();
        }

        // try to allocate on existing heap
        if let Ok(ptr) = self.inner.get().unwrap().lock().allocate_first_fit(layout) {
            return ptr.as_ptr();
        }

        // allocation failed -> try to extend heap and allocate
        while self.request().is_ok() {
            if let Ok(ptr) = self.inner.get().unwrap().lock().allocate_first_fit(layout) {
                return ptr.as_ptr();
            }
        }

        // could not allocate (likely heap size limit)
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        if !self.inner.is_initialized() {
            self.init();
        }
        if let Some(nn_ptr) = NonNull::new(ptr) {
            unsafe { self.inner.get().unwrap().lock().deallocate(nn_ptr, layout) };
        }
    }
}
