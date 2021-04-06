use crate::cell::Cell;
use std::usize;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    // Cloning a box clones the T.
    inner: *const RcInner<T>,
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
            inner: Box::into_raw(inner),
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

impl<T> Clone for Rc<T> {
    // Increasing the reference count, but not copying the inner value. Only one of the inner value.
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner }
    }
}
