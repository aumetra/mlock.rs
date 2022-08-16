use core::{ffi::c_void, mem};
use std::io;

mod raw {
    use core::ffi::c_void;

    extern "C" {
        pub fn VirtualLock(lpAddress: *const c_void, dwSize: usize) -> i32;
        pub fn VirtualUnlock(lpAddress: *const c_void, dwSize: usize) -> i32;
    }
}

pub fn mlock<T>(item: &T) -> io::Result<()> {
    let len = mem::size_of::<T>();
    let ret_code = unsafe { raw::VirtualLock(item as *const T as *const c_void) };

    if ret_code == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn munlock<T>(item: &T) -> io::Result<()> {
    let len = mem::size_of::<T>();
    let ret_code = unsafe { raw::VirtualUnlock(item as *const T as *const c_void, len) };

    if ret_code == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
