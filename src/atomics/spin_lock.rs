use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

/*
* We use acquire and release memory ordering to make sure that every unlock()
* call establishes a happens-before relationship with the lock() calls that follow
*/
impl<T> SpinLock<T> {

    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value)
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {

    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {

    fn drop(&mut self) {
        self.lock.locked.store(false, Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use crate::atomics::spin_lock::SpinLock;

    #[test]
    fn test_1() {

        fn main() {
            let x = SpinLock::new(Vec::new());

            thread::scope(|s| {
                s.spawn(|| x.lock().push(1));
                s.spawn(|| {
                    let mut g = x.lock();
                    g.push(2);
                    g.push(2);
                });
            });

            let g = x.lock();
            assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
        }
    }
}