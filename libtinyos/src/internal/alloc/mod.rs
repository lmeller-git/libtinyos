use crate::{println, request_heap};

use conquer_once::spin::OnceCell;
use core::{
    alloc::GlobalAlloc,
    ptr::{NonNull, null_mut},
};
use linked_list_allocator::{LockedHeap, align_up_size};

const START_HEAP_SIZE: usize = 1024 * 100;

const ALIGN: usize = 4096; // as kernel pages are 4 KiB currently

#[global_allocator]
pub(crate) static GLOBAL_ALLOC: EnsureInitAlloc = EnsureInitAlloc::empty();

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AllocData {
    free: usize,
    used: usize,
}

pub fn alloc_data() -> Option<AllocData> {
    let inner = GLOBAL_ALLOC.inner.get()?.lock();
    Some(AllocData {
        free: inner.free(),
        used: inner.used(),
    })
}

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
        let size = self.inner.get().unwrap().lock().size() * 2;
        let size = align_up_size(size, ALIGN);

        let ptr = request_heap(size);
        if ptr.is_null() {
            return Err(());
        }
        unsafe { self.inner.get().unwrap().lock().extend(size) };
        Ok(())
    }
}

unsafe impl GlobalAlloc for EnsureInitAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if !self.inner.is_initialized() {
            self.init();
        }

        let inner = self.inner.get().unwrap();

        // Linked List allocator fails if requested size == free size
        // in that case we simply try to allocate a bit more, which will trigger an extension
        let layout = if layout.size() == inner.lock().free() {
            let Ok(layout) =
                core::alloc::Layout::from_size_align(layout.size() + 1, layout.align())
            else {
                return null_mut();
            };
            layout
        } else {
            layout
        };

        // try to allocate on existing heap
        let ptr = unsafe { inner.alloc(layout) };
        if !ptr.is_null() {
            return ptr;
        }

        // allocation failed -> try to extend heap and allocate
        while self.request().is_ok() {
            let ptr = unsafe { inner.alloc(layout) };
            if !ptr.is_null() {
                return ptr;
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
