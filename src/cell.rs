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
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // // this will not work
    #[test]
    fn bad_implementation() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new([1, 1000000000]));
        let x1 = Arc::clone(&x);
        let first = std::thread::spawn(move || {
            x1.set([2, 1000000000]);
        });
        let x2 = Arc::clone(&x);
        let second = std::thread::spawn(move || {
            x2.set([3, 1000000000]);
        });
        first.join().unwrap();
        second.join().unwrap();
        println!("{:?} ", x.get(),)
    }
}
