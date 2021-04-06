use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// We get this with UnsafeCell;
// impl<T> !Sync for Cell<T> {}
// It's thread safe.

unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }
    // We give out a copy of it.
    // Returning a reference to self; erasing the copy trait bound.
    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // // this will not work
    #[test]
    fn bad_implementation() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        std::thread::spawn(move || {
            x1.set(43);
        });
        let x2 = Arc::clone(&x);
        std::thread::spawn(move || {
            x2.set(44);
        });
    }

    #[test]
    fn bad2() {
        let x = Cell::new(vec![42]);
        // Get only returns a copy and not a reference. So this should not work...
        let first = &x.get()[0];
        x.set(vec![]);
        println!("{}", first);
    }
}
