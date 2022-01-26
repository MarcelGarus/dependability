use alloc::vec::Vec;
use core::cell::UnsafeCell;

pub(crate) struct PriorityQueue<I, P: Ord + Copy> {
    items: UnsafeCell<Vec<(I, P)>>,
}

impl<I, P: Ord + Copy> PriorityQueue<I, P> {
    pub fn new() -> Self {
        Self {
            items: UnsafeCell::new(Vec::new()),
        }
    }
    pub fn push(&self, item: I, priority: P) {
        unsafe {
            (*self.items.get()).push((item, priority));
        }
    }
    pub fn pop(&self) -> Option<(I, P)> {
        unsafe {
            let minimum = (*self.items.get()).iter().map(|(_, p)| p).min()?;
            let index = (*self.items.get())
                .iter()
                .position(|(_, p)| p == minimum)
                .unwrap();
            Some((*self.items.get()).remove(index))
        }
    }
    pub fn is_empty(&self) -> bool {
        unsafe { (*self.items.get()).is_empty() }
    }
}

unsafe impl<I, P: Ord + Copy> Send for PriorityQueue<I, P> {}
unsafe impl<I, P: Ord + Copy> Sync for PriorityQueue<I, P> {}
