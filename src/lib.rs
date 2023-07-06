use std::cmp;
use std::mem;
use std::ptr::{self, NonNull};

pub struct MyBox<T>(NonNull<T>);

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        assert_ne!(
            mem::size_of::<T>(),
            0,
            "we can't handle zero-sized types yet"
        );
        let mut memptr: *mut T = ptr::null_mut();

        unsafe {
            let ret = libc::posix_memalign(
                (&mut memptr as *mut *mut T).cast(),
                cmp::max(mem::align_of::<T>(), mem::size_of::<usize>()),
                mem::size_of::<T>(),
            );

            assert_eq!(ret, 0, "libc::posix_memalign returned non-zero value");
        }

        let ptr =
            NonNull::new(memptr).expect("should be correct if libc::posix_memalign is correct");

        unsafe {
            ptr.as_ptr().write(value);
        }

        MyBox(ptr)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

unsafe impl<T> Send for MyBox<T> where T: Send {}
unsafe impl<T> Sync for MyBox<T> where T: Sync {}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.0.as_ptr().cast()) }
    }
}

#[test]
fn test_my_box() {
    let mut a = MyBox::new(4);
    assert_eq!(*a, 4);

    *a = 20;
    assert_eq!(*a, 20);
}
