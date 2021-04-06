use std::cell::UnsafeCell;

enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: RefState,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: RefState::Unshared,
        }
    }
    pub fn borrow(&self) -> Option<&T> {
        match self.state {
            RefState::Unshared => {
                // self.state = RefState::Shared(1);
                Some(unsafe { &*self.value.get() })
            }
            RefState::Shared(n) => {
                // self.state = RefState::Shared(n + 1);
                Some(unsafe { &*self.value.get() })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        if let RefState::Unshared = self.state {
            // self.state = RefState::Exclusive;
            Some(unsafe { &mut *self.value.get() })
        } else {
            None
        }
    }
}
