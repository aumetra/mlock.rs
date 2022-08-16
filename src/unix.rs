use core::{ffi::c_void, mem};
use std::io;

mod raw {
    use core::ffi::c_void;

    extern "C" {
        pub fn mlock(addr: *const c_void, len: usize) -> i32;
        pub fn munlock(addr: *const c_void, len: usize) -> i32;
    }
}

pub fn mlock<T>(item: &T) -> io::Result<()> {
    let len = mem::size_of::<T>();
    let ret_code = unsafe { raw::mlock(item as *const T as *const c_void, len) };

    if ret_code == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn munlock<T>(item: &T) -> io::Result<()> {
    let len = mem::size_of::<T>();
    let ret_code = unsafe { raw::munlock(item as *const T as *const c_void, len) };

    if ret_code == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
