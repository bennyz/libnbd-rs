#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use std::ffi::{CStr, CString};

    use super::*;

    #[test]
    fn test() {
        unsafe {
            let handle = nbd_create();

            let c_str = CString::new("hello").unwrap();
            nbd_set_handle_name(handle, c_str.as_ptr());
            assert_eq!(
                CStr::from_ptr(nbd_get_handle_name(handle))
                    .to_str()
                    .unwrap(),
                "hello"
            );
        }
    }
}
