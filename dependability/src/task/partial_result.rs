use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref SLOT: Arc<Mutex<PartialResultSlot>> = PartialResultSlot::new();
}

unsafe impl Send for PartialResultSlot {}
pub struct PartialResultSlot {
    inner: Cell<Option<*const ()>>,
}

impl PartialResultSlot {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            inner: Cell::new(None),
        }))
    }

    pub fn set<T>(&self, value: T) {
        let pointer = &value as *const T;
        self.inner.set(Some(pointer.cast()));
    }

    pub fn get<T>(&self) -> Option<T> {
        self.inner.replace(None).map(|pointer| {
            let pointer = pointer.cast::<T>();
            unsafe { *pointer }
        })
    }
}
