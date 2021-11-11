use std::{env, ffi::CStr};

use libnbd_rs::{bindings::nbd_extent_callback, bindings::LIBNBD_STATE_ZERO as STATE_ZERO, c_void};

#[derive(Debug, Default)]
#[repr(C)]
struct ExtentCallbackData<'a> {
    extents: Box<&'a [u32]>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let uri = &args[1];

    let mut h = libnbd_rs::NbdHandle::create();
    h.add_meta_context("base:allocation");
    h.connect_uri(uri);

    let size = h.get_size();

    let mut offset = 0;
    while offset < size {
        let length = std::cmp::min(size - offset, libnbd_rs::SEGMENT_SIZE as i64);
        let mut data = ExtentCallbackData::default();
        let cb = nbd_extent_callback {
            callback: Some(callback),
            user_data: &mut data as *mut ExtentCallbackData as *mut c_void,
            free: Some(libnbd_rs::free_callback),
        };

        h.block_status(length.try_into().unwrap(), offset.try_into().unwrap(), cb);
        let mut ext_start = offset;
        for mut i in 0..data.extents.len() - 1 {
            let ext_len = data.extents[i];
            let ext_zero = data.extents[i + 1] & STATE_ZERO == STATE_ZERO;
            println!("offest={} length={} zero={}", ext_start, ext_len, ext_zero);
            ext_start += ext_len as i64;

            i += 2;
        }

        offset += libnbd_rs::SEGMENT_SIZE as i64;
    }

    h.close();
}

unsafe extern "C" fn callback(
    user_data: *mut ::std::os::raw::c_void,
    metacontext: *const ::std::os::raw::c_char,
    _offset: u64,
    entries: *mut u32,
    nr_entries: libnbd_rs::bindings::size_t,
    error: *mut ::std::os::raw::c_int,
) -> i32 {
    let c_str: &CStr = CStr::from_ptr(metacontext);

    if c_str.to_str().unwrap() == "base:allocation" {
        let rs_entries = std::slice::from_raw_parts(entries, nr_entries as usize);
        let data = user_data as *mut ExtentCallbackData;
        (*data).extents = Box::from(rs_entries);
    }

    return *error;
}