// src/data_structures/vec/vector.rs

use std::alloc::{Layout, alloc, realloc};
use std::mem;
use std::ptr;

/*
    Dynamic Vector - Constraints
    Basic Requirements:

    Dynamic storage - grows automatically as needed - OK
    Index access - O(1) to access element at any position - OK
    Insertion at end - O(1) amortized (push) - OK
    Removal from end - O(1) (pop) - OK
    Resizing - when capacity is reached, must allocate more memory - OK

    Mandatory Operations:

    push(value) - add element at the end - OK
    pop() - remove element from the end - OK
    get(index) - access element by index - OK
    set(index, value) - modify element
    len() - return number of elements - OK
    capacity() - return allocated space - OK
    is_empty() - check if empty - OK

    Search Algorithms
*/

pub struct Vector<T> {
    pointer: *mut T,
    len: usize,      // current size
    capacity: usize, // allocated space
}

impl<T> Vector<T> {
    /// Search for the element traversing the entire array, returns the index of the element
    /// Time complexity - O(n)
    /// Space complexity - O(1)
    pub fn linear_search(&self, target: T) -> Option<usize> {

    }

    /// Creates a new empty Vector
    pub fn new() -> Self {
        Vector {
            pointer: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    /// Creates a Vector with a specified initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let layout = Layout::from_size_align(capacity * mem::size_of::<T>(), mem::align_of::<T>())
            .expect("Invalid layout");
        let pointer = unsafe { alloc(layout) as *mut T };
        if pointer.is_null() {
            panic!("Failed to allocate memory for Vector");
        }

        Vector {
            pointer,
            len: 0,
            capacity,
        }
    }

    /// Adds an element to the end of the Vector
    ///
    /// # Safety
    /// This function is unsafe because it manipulates raw pointers directly.
    /// Ensure the Vector is properly initialized before calling this method.
    pub fn push(&mut self, item: T) {
        // If capacity is reached, we need to grow
        if self.capacity == self.len {
            self.grow();
        }

        // Points to the next empty position
        unsafe {
            let position = self.pointer.add(self.len);
            position.write(item);
        }

        self.len += 1;
    }

    /// Internal method that grows the capacity of the Vector
    fn grow(&mut self) {
        // Define the new capacity (double, or start at 1 if was 0)
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        // If there was no previous allocation, allocate new space
        if self.capacity == 0 {
            let layout =
                Layout::from_size_align(new_capacity * mem::size_of::<T>(), mem::align_of::<T>())
                    .expect("Invalid layout");
            self.pointer = unsafe { alloc(layout) as *mut T };
            if self.pointer.is_null() {
                panic!("Failed to allocate memory for Vector");
            }
        } else {
            // If there was already an allocation, reallocate with new size
            let old_layout =
                Layout::from_size_align(self.capacity * mem::size_of::<T>(), mem::align_of::<T>())
                    .expect("Invalid layout");

            let new_layout =
                Layout::from_size_align(new_capacity * mem::size_of::<T>(), mem::align_of::<T>())
                    .expect("Invalid layout");

            let new_pointer = unsafe {
                realloc(self.pointer as *mut u8, old_layout, new_layout.size()) as *mut T
            };
            if new_pointer.is_null() {
                panic!("Failed to reallocate memory for Vector");
            }
            self.pointer = new_pointer;
        }

        self.capacity = new_capacity;
        println!("Vector grew to capacity: {}", new_capacity);
    }

    /// Returns the number of elements in the Vector
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the allocated capacity of the Vector
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Checks if the Vector is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                let position = self.pointer.add(index);
                Some(&*position)
            }
        }
    }

    /// Pop the last item
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.len -= 1;
            unsafe {
                let position = self.pointer.add(self.len);
                Some(ptr::read(position))
            }
        } else {
            None
        }
    }

    /// Modify element based in the index
    pub fn set(&mut self,  index: usize, value: T,) -> Option<T> {
        if index < self.len  {
            unsafe {
                let position = self.pointer.add(index);
                Some(position.replace(value))
            }
        }else {
            None
        }
    }
}

/// Implement Drop to free memory when Vector goes out of scope
impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.capacity != 0 && !self.pointer.is_null() {
            let layout =
                Layout::from_size_align(self.capacity * mem::size_of::<T>(), mem::align_of::<T>())
                    .expect("Invalid layout");
            unsafe {
                std::alloc::dealloc(self.pointer as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_vec() {
        let vec: Vector<i32> = Vector::new();
        assert_eq!(vec.len, 0);
        assert_eq!(vec.capacity, 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn should_create_with_capacity() {
        let vec: Vector<i32> = Vector::with_capacity(10);
        assert_eq!(vec.len, 0);
        assert_eq!(vec.capacity, 10);
        assert!(vec.is_empty());
    }

    #[test]
    fn should_push_single_element() {
        let mut vec: Vector<i32> = Vector::with_capacity(5);

        vec.push(42);

        assert_eq!(vec.len, 1);
        assert_eq!(vec.capacity, 5);
        assert!(!vec.is_empty());
    }

    #[test]
    fn should_push_multiple_elements() {
        let mut vec: Vector<i32> = Vector::with_capacity(5);

        vec.push(10);
        vec.push(20);
        vec.push(30);

        assert_eq!(vec.len, 3);
        assert_eq!(vec.capacity, 5);
    }

    #[test]
    fn should_push_triggers_growth() {
        let mut vec: Vector<i32> = Vector::with_capacity(2);

        vec.push(1);
        vec.push(2);
        assert_eq!(vec.len, 2);
        assert_eq!(vec.capacity, 2);

        vec.push(3); // grow here
        assert_eq!(vec.len, 3);
        assert_eq!(vec.capacity, 4);
    }

    #[test]
    fn should_push_from_empty() {
        let mut vec: Vector<i32> = Vector::new();

        vec.push(100); // Should grow from 0 to 1
        assert_eq!(vec.len, 1);
        assert_eq!(vec.capacity, 1);

        vec.push(200); // Should grow from 1 to 2
        assert_eq!(vec.len, 2);
        assert_eq!(vec.capacity, 2);
    }

    #[test]
    fn should_get_valid_index() {
        let mut vec: Vector<i32> = Vector::with_capacity(5);
        vec.push(10);
        vec.push(20);
        vec.push(30);

        assert_eq!(*vec.get(0).unwrap(), 10);
        assert_eq!(*vec.get(1).unwrap(), 20);
        assert_eq!(*vec.get(2).unwrap(), 30);
    }

    #[test]
    fn should_get_invalid_index() {
        let mut vec: Vector<i32> = Vector::with_capacity(3);
        vec.push(10);

        assert_eq!(vec.get(10), None);
        assert_eq!(vec.get(1), None);
    }

    #[test]
    fn should_pop_valid() {
        let mut vec: Vector<i32> = Vector::with_capacity(5);
        vec.push(10);
        vec.push(20);
        vec.push(30);

        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn should_pop_empty() {
        let mut vec: Vector<i32> = Vector::new();
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn should_set_valid(){
        let mut vec: Vector<i32> = Vector::with_capacity(5);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.set(1, 100);

        assert_eq!(vec.get(1), Some(&100))
    }

    #[test]
    fn should_set_invalid(){
        let mut vec: Vector<i32> = Vector::with_capacity(5);

        assert_eq!(vec.set(1, 100), None)
    }

    #[test]
    fn should_find_element_linear_search(){

    }

    #[test]
    fn should_return_none_if_no_match_linear_search(){
        Elemento no início
        Elemento no fim
        Elemento no meio
    }

    #[test]
    fn should_return_none_if_array_is_empty_linear_search(){

    }
}
