use core::{
    alloc::{Allocator, Layout},
    ptr::NonNull,
};

use crate::RestrictiveAlloc;

struct InnerVec<T> {
    ptr: NonNull<T>,
    layout: Layout,
}

pub struct Vec<A, const MAX: usize, T>
where
    A: Allocator,
{
    inner: InnerVec<T>,
    capacity: usize,
    allocator: RestrictiveAlloc<A>,
}

impl<A: Allocator, const MAX: usize, T> Vec<A, MAX, T> {
    pub fn new(allocator: A) -> Self {
        Self {
            inner: InnerVec {
                ptr: allocator.allocate(Layout::new::<T>()).unwrap().cast(),
                layout: Layout::new::<T>(),
            },
            capacity: 0,
            allocator: RestrictiveAlloc::new(allocator, MAX),
        }
    }

    pub async fn grow(&mut self, new_layout: Layout) {
        let InnerVec { ptr, layout } = self.inner;
        let new_ptr = unsafe { self.allocator.grow(ptr.cast(), layout, new_layout).unwrap() };
        self.inner.layout = new_layout;
        self.inner.ptr = new_ptr.cast();
    }

    pub async fn push(&mut self, element: T) {
        self.grow(Layout::array::<T>(self.capacity + 1).unwrap())
            .await;
        unsafe { self.inner.ptr.as_ptr().add(self.capacity).write(element) };
        self.capacity += 1;
    }

    pub fn get(&self, idx: usize) -> T {
        unsafe { self.inner.ptr.as_ptr().add(idx).read() }
    }
}
