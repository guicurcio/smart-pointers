use std::cell::UnsafeCell;

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    reference: isize,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            reference: 0,
        }
    }
    pub fn borrow(&self) -> Option<&T> {
        None
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        None
    }
}
