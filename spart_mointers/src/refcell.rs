use std::cell::{self, UnsafeCell}:UnsafeCell;

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    reference: isize,
}
