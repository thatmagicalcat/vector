use std::alloc;
use std::ptr;
use std::ops::Index;
use std::ops::Drop;

use super::vector_iterator::VectorIterator;

pub struct Vector<T> {
    pub(super) ptr: *mut T,
    pub(super) len: usize,
    pub(super) capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        let capacity = 1;
        let ptr = unsafe { alloc::alloc(alloc::Layout::array::<T>(capacity).unwrap()) as *mut T };

        Self {
            capacity,
            ptr,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len >= self.capacity {
            self.reserve(self.capacity + 4);
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { ptr::read(self.ptr.add(self.len) as *const T) })
        }
    }

    pub fn iter(&self) -> VectorIterator<T> {
        VectorIterator::new(self)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn reserve(&mut self, new_capacity: usize) {
        if new_capacity <= self.capacity {
            return;
        }

        let new_layout = alloc::Layout::array::<T>(new_capacity).expect("Failed to create layout");
        let new_ptr = unsafe { alloc::alloc(new_layout) as *mut T };

        unsafe {
            ptr::copy_nonoverlapping(self.ptr, new_ptr, self.capacity);
        }

        unsafe { alloc::dealloc(self.ptr as *mut _, alloc::Layout::array::<T>(self.capacity).unwrap()); }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }

}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds, size is: {} and index is: {}",
                self.len, index
            );
        }

        unsafe { &*self.ptr.add(index) }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                std::ptr::drop_in_place(self.ptr.add(i));
            }
        }

        unsafe {
            std::alloc::dealloc(
                self.ptr as *mut _,
                std::alloc::Layout::array::<T>(self.capacity).unwrap(),
            );
        }
    }
}
