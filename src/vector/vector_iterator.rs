use std::ptr;

use super::vector::Vector;

pub struct VectorIterator<T> {
    ptr: *const T,
    len: usize,
    offset: usize, // used to keep track of the current item
}

impl<T> VectorIterator<T> {
    pub(super) fn new(vector: &Vector<T>) -> Self {
        Self {
            ptr: vector.ptr,
            len: vector.len,
            offset: 0
        }
    }
}

impl<T> Iterator for VectorIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.len {
            None
        } else {
            let ptr = unsafe { ptr::read(self.ptr.add(self.offset)) };
            self.offset += 1;

            Some(ptr)
        }
    }
}
