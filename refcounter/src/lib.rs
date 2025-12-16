use std::sync::atomic::{AtomicIsize, Ordering};

/// This type allows for doing both synchronized and non-synchronized
/// reference counting. This is useful when reference counting can be
/// precise as with Koka's Perceus.
///
/// In fact, we follow Perceus's model of using a positive ref count
/// to indicate a non-atomic access and a negative ref count as an 
/// indication of atomic reference counting.
///
/// The only caveat is that we have to obey Rust's aliasing (mutability)
/// rules. As a result, all operations require being `&mut` to function.
/// However, this should be acceptable as this type is intended to be
/// embedded into types that are already a `*mut` so this should be fine.
#[repr(C)]
pub struct RefCounter(AtomicIsize);


impl RefCounter {
    
    /// Creates a new `RefCounter` starting with count of 1
    pub fn new() -> RefCounter {
        RefCounter(AtomicIsize::new(1))
    }
    
    /// Creates a new `RefCounter` starting with count of -1
    pub fn new_atomic() -> RefCounter {
        RefCounter(AtomicIsize::new(-1))
    }
    
    /// Makes a `RefCounter` shared.
    ///
    /// This is a one way function.
    pub fn make_shared(&mut self) {
        let value = self.fetch_value();
        if value > 0 {
            unsafe {
                *self.0.as_ptr() = -*self.0.as_ptr();
            }
        }
    }

    /// Makes a `RefCounter` no longer shared.
    ///
    /// This function only succeeds if the reference count is equal to
    /// -1 as this indicates that it a unique reference.
    ///
    /// Returns `true` when reference count is equal to -1
    /// Returns `false` otherwise
    pub fn make_unshared(&mut self) -> bool {
        let value = self.fetch_value();
        if value == -1 {
            unsafe {
                *self.0.as_ptr() = -*self.0.as_ptr();
            }
            true
        } else {
            false
        }
    }

    /// Increments the reference count of the counter.
    ///
    /// This function is only atomic if the count < 0.
    pub fn increment(&mut self) {
        let value = self.fetch_value();
        assert_ne!(value, 0, "Incrementing a reference count of zero!");
        if value < 0 {
            self.0.fetch_add(-1, Ordering::Relaxed);
        } else {
            unsafe {
                *self.0.as_ptr() += 1;
            }
        }
    }
    
    /// Decrements the reference count of the counter.
    ///
    /// This function is only atomic if the count < 0.
    ///
    /// Returns the new value.
    pub fn decrement(&mut self) -> isize {
        let value = self.fetch_value();
        assert_ne!(value, 0, "Decrementing a reference count of zero!");
        if value < 0 {
            // since we fetch, then add, we need to add 1 to the result
            // to get the right value.
            self.0.fetch_add(1, Ordering::Relaxed) + 1
        } else {
            unsafe {
                *self.0.as_ptr() -= 1;
                *self.0.as_ptr()
            }
        }
    }
   
    /// Fetches the current reference count.
    ///
    /// Normally this operation should be atomic.
    /// However, since reference counting holds the invariant that we 
    /// won't be zero, this is perfectly safe to do non-atomically.
    pub fn fetch_value(&mut self) -> isize {
        unsafe {
            *self.0.as_ptr()
        }
    }
    
}
