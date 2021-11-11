#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use bindings::nbd_extent_callback;
use std::ffi::{CStr, CString};
pub mod bindings;

pub type c_void = libc::c_void;
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

    pub fn add_meta_context(&mut self, context: &str) {
        unsafe {
            let uri = CString::new(context).unwrap();
            bindings::nbd_add_meta_context(self.handle, uri.as_ptr());
        }
    }

    pub fn connect_uri(&mut self, uri: &str) {
        unsafe {
            let uri = CString::new(uri).unwrap();
            let r = bindings::nbd_connect_uri(self.handle, uri.as_ptr());
        }
    }

    pub fn get_size(&mut self) -> i64 {
        unsafe {
            let s = bindings::nbd_get_size(self.handle);
            return s;
        }
    }

    pub fn block_status(&mut self, count: u64, offset: u64, cb: nbd_extent_callback) -> i32 {
        unsafe {
            let r = bindings::nbd_block_status(self.handle, count, offset, cb, 0);
            return r;
        }
    }

    pub fn close(&mut self) {
        unsafe {
            bindings::nbd_close(self.handle);
        }
    }
}

unsafe extern "C" fn callback(
    user_data: *mut ::std::os::raw::c_void,
    metacontext: *const ::std::os::raw::c_char,
    offset: u64,
    entries: *mut u32,
    nr_entries: bindings::size_t,
    error: *mut ::std::os::raw::c_int,
) -> i32 {
    let data = user_data as *mut ExtentCallbackData;
    (*data).extents += 1;
    println!("block status callback");
    println!("userdata: {:?}", *data);
    println!("metacontext: {:?}", *metacontext);
    println!("offset: {}", offset);
    println!("entries: {:?}", entries);
    println!("nr_entries: {:?}", nr_entries);
    println!("err: {:?}", *error);
    return *error;
}

pub unsafe extern "C" fn free_callback(user_data: *mut ::std::os::raw::c_void) {}

#[derive(Debug)]
struct ExtentCallbackData {
    extents: u64,
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
        handle.add_meta_context("base:allocation");
        handle.connect_uri(uri);
        let data: *mut c_void = &mut data as *mut RustData as *mut c_void;
        let cb = nbd_extent_callback {
            callback: Some(callback),
            user_data: data,
            free: Some(free_callback),
        };

        let block_status = handle.block_status(1000, 1, cb);
        println!("block status: {:?}", block_status);

        // based on fedora cloud qcow2
        assert_eq!(handle.get_size(), 5368709120);
    }
}
