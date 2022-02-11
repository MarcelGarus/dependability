#![no_std]
#![feature(allocator_api)]

pub mod collections;

extern crate alloc;

use core::{
    alloc::{AllocError, Allocator},
    cell::Cell,
    task::Poll,
};

pub struct RestrictiveAlloc<A: Allocator> {
    inner: A,
    max: usize,
    allocated: Cell<usize>,
}

impl<A: Allocator> RestrictiveAlloc<A> {
    pub fn new(inner: A, max: usize) -> Self {
        Self {
            inner,
            max,
            allocated: Cell::new(0),
        }
    }

    pub fn try_allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Poll<Result<core::ptr::NonNull<[u8]>, AllocError>> {
        let new_allocated = self.allocated.get() + layout.size();
        if new_allocated < self.max {
            Poll::Ready(self.allocate(layout))
        } else {
            Poll::Pending
        }
    }
}

unsafe impl<A: Allocator> Allocator for RestrictiveAlloc<A> {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        let new_allocated = self.allocated.get() + layout.size();
        if new_allocated < self.max {
            let ptr = self.inner.allocate(layout)?;
            self.allocated.set(new_allocated);
            Ok(ptr)
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        self.inner.deallocate(ptr, layout);
        self.allocated.set(self.allocated.get() - layout.size());
    }
}
