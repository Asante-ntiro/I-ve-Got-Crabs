use std::cell::UnsafeCell;

struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell <T> {
    fn new(value: T) -> Self {
        Cell {
            value
        }
    }
    
}
