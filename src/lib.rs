#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use std::ffi::{CStr, CString};
mod bindings;

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

    pub fn connect_uri(&mut self, uri: &str) {
        unsafe {
            let uri = CString::new(uri).unwrap();
            let r = bindings::nbd_connect_uri(self.handle, uri.as_ptr());
            println!("rc: {}", r);
        }
    }

    pub fn get_size(&mut self) -> i64 {
        unsafe {
            let s = bindings::nbd_get_size(self.handle);
            println!("size: {}", s);
            return s;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_size() {
        // unsafe {
        //     let handle = bindings::nbd_create();
        //     let uri = CString::new("nbd://localhost:10809").unwrap();
        //     let r = bindings::nbd_connect_uri(handle, uri.as_ptr());
        //     let size = bindings::nbd_get_size(handle);
        //     assert_eq!(size, 10485760);
        // }
        let mut handle = NbdHandle::create();
        let uri = "nbd://localhost:10809";
        handle.connect_uri(uri);
        assert_eq!(handle.get_size(), 10485760);
    }
}
