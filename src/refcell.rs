use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: crate::cell::Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref {
                    reference_to_refcell: self,
                })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref {
                    reference_to_refcell: self,
                })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            // This is the only reference that can be given.
            self.state.set(RefState::Exclusive);
            Some(RefMut {
                mut_reference_to_refcell: self,
            })
        } else {
            None
        }
    }
}

// Points to the lifetime of the refcell.
pub struct Ref<'refcell, T> {
    reference_to_refcell: &'refcell RefCell<T>,
}
pub struct RefMut<'refcell, T> {
    mut_reference_to_refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        // decrement the reference ount.
        // state must be shared.
        match self.reference_to_refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.reference_to_refcell.state.set(RefState::Unshared),
            // If it's shared, then decrement it by one when dropping it.
            RefState::Shared(n) => self.reference_to_refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &mut *self.reference_to_refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        // decrement the reference ount.
        // state must be shared.
        match self.mut_reference_to_refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            // When  dropped we know that it's unshared.
            RefState::Exclusive => self.mut_reference_to_refcell.state.set(RefState::Unshared),
        }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mut_reference_to_refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mut_reference_to_refcell.value.get() }
    }
}
