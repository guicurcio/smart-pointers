use crate::cell::Cell;
use std::ptr::NonNull;
use std::usize;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    // Cloning a box clones the T.
    // Not safe to mutate this pointer -- this mutable pointer.
    inner: NonNull<RcInner<T>>,
    // refcount?
    // refcount: usize,
    // It the value that it's shared among all the copies.
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    // Increasing the reference count, but not copying the inner value. Only one of the inner value.
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // Inner is a pointer to box +4
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // Only reference.
            // Inner must go away.
            drop(inner);
            unsafe {
                // We drop the box and any pointer to that box is invalid.
                let _ = Box::from_raw(self.inner.as_ptr());
            }
        } else {
            inner.refcount.set(c - 1)
        }
    }
}
