#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use bindings::nbd_extent_callback;
use std::ffi::{CStr, CString};
pub mod bindings;

pub type c_void = ::std::os::raw::c_void;
pub const SEGMENT_SIZE: usize = 1024 * 1024 * 1024;

pub struct NbdExtentCallback {
    callback: bindings::nbd_extent_callback,
}
pub struct NbdHandle {
    handle: *mut bindings::nbd_handle,
}

impl NbdHandle {
    pub fn create() -> NbdHandle {
        unsafe {
            NbdHandle {
                handle: bindings::nbd_create(),
            }
        }
    }

    pub fn add_meta_context(&mut self, context: &str) -> i32 {
        let uri = CString::new(context).unwrap();
        let ptr = uri.into_raw();
        let mut r = 0;
        unsafe {
            r = bindings::nbd_add_meta_context(self.handle, ptr);

            // Make rust free allocated memory
            let _ = CString::from_raw(ptr);
        }

        return r;
    }

    pub fn connect_uri(&mut self, uri: &str) -> i32 {
        let uri = CString::new(uri).unwrap();
        let ptr = uri.into_raw();
        let mut r = 0;
        unsafe {
            r = bindings::nbd_connect_uri(self.handle, ptr);

            // Make rust free allocated memory
            let _ = CString::from_raw(ptr);
        }

        return r;
    }

    pub fn set_handle_name(&mut self, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        let ptr = name.into_raw();
        let mut r = 0;
        unsafe {
            r = bindings::nbd_set_handle_name(self.handle, ptr);
            let _ = CString::from_raw(ptr);
        }

        return r;
    }

    pub fn get_handle_name(&mut self) -> String {
        unsafe {
            let handle_name = CString::from_raw(bindings::nbd_get_handle_name(self.handle));
            handle_name.to_str().unwrap().to_string()
        }
    }

    pub fn connect_command(&mut self, mut cmd: &[&str]) -> i32 {
        let mut r = 0;
        unsafe {
            let mut cmd_ptr = Vec::new();
            for c in cmd.iter() {
                let c = CString::new(*c).unwrap();
                cmd_ptr.push(c.into_raw());
            }
            r = bindings::nbd_connect_command(self.handle, cmd_ptr.as_mut_slice().as_mut_ptr());

            for p in cmd_ptr.iter() {
                let _ = CString::from_raw(*p);
            }
        }

        return r;
    }

    pub fn get_size(&mut self) -> i64 {
        unsafe { bindings::nbd_get_size(self.handle) }
    }

    pub fn block_status(&mut self, count: u64, offset: u64, cb: nbd_extent_callback) -> i32 {
        unsafe { bindings::nbd_block_status(self.handle, count, offset, cb, 0) }
    }

    pub fn close(&mut self) {
        unsafe {
            bindings::nbd_close(self.handle);
        }
    }
}

pub unsafe extern "C" fn free_callback(user_data: *mut c_void) {}

#[derive(Debug)]
struct ExtentCallbackData {
    extents: u64,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_handle_name() {
        let mut handle = NbdHandle::create();
        handle.set_handle_name("test");
        let expected_name = handle.get_handle_name();
        handle.close();
        assert_eq!(expected_name, "test");
    }

    #[test]
    fn test_size() {
        let mut handle = NbdHandle::create();
        let expected_size = 1048576;
        handle.connect_command(&[
            "nbdkit",
            "-s",
            "--exit-with-parent",
            "null",
            &format!("size={}", expected_size),
        ]);

        let size = handle.get_size();
        handle.close();
        assert_eq!(size, expected_size);
    }
}
