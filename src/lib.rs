#![no_std]
pub(crate) mod syscalls;
use core::{
    alloc::GlobalAlloc,
    ptr::{NonNull, null_mut},
};

use conquer_once::spin::OnceCell;
use linked_list_allocator::LockedHeap;
pub use syscalls::funcs::*;
extern crate alloc;

const START_HEAP_SIZE: usize = 1024 * 500;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    exit(2)
}

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
}

unsafe impl GlobalAlloc for EnsureInitAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if !self.inner.is_initialized() {
            self.init();
        }
        match self.inner.get().unwrap().lock().allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => null_mut(),
        }
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
