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
            let (index, (_, _)) = (*self.items.get())
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, p))| p)?;

            Some((*self.items.get()).swap_remove(index))
        }
    }
    pub fn is_empty(&self) -> bool {
        unsafe { (*self.items.get()).is_empty() }
    }
}

unsafe impl<I, P: Ord + Copy> Send for PriorityQueue<I, P> {}
unsafe impl<I, P: Ord + Copy> Sync for PriorityQueue<I, P> {}
