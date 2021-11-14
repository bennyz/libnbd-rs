use std::{env, ffi::CStr};

use libnbd_rs::{bindings::nbd_extent_callback, bindings::LIBNBD_STATE_ZERO as STATE_ZERO, c_void};

#[derive(Debug, Default)]
#[repr(C)]
struct ExtentCallbackData<'a> {
    extents: Box<&'a [u32]>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./map <uri>");
        return;
    }

    let uri = &args[1];

    let mut h = libnbd_rs::NbdHandle::create();
    h.add_meta_context("base:allocation");
    h.connect_uri(uri);

    let size = h.get_size();

    for offset in (0..size).step_by(libnbd_rs::SEGMENT_SIZE as usize) {
        let length = std::cmp::min(size - offset, libnbd_rs::SEGMENT_SIZE as i64);
        let mut data = ExtentCallbackData::default();
        let cb = nbd_extent_callback {
            callback: Some(callback),
            user_data: &mut data as *mut ExtentCallbackData as *mut c_void,
            free: Some(libnbd_rs::free_callback),
        };

        h.block_status(length.try_into().unwrap(), offset.try_into().unwrap(), cb);
        let mut ext_start = offset;
        for i in (0..data.extents.len() - 1).step_by(2) {
            let ext_len = data.extents[i];
            let ext_zero = data.extents[i + 1] & STATE_ZERO == STATE_ZERO;
            println!("offset={} length={} zero={}", ext_start, ext_len, ext_zero);

            ext_start += ext_len as i64;
        }
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

    if *error != 0 {
        panic!("error {}", *error);
    }

    match c_str.to_str() {
        Ok(s) => {
            if s == "base:allocation" {
                let rs_entries = std::slice::from_raw_parts(entries, nr_entries as usize);
                let data = user_data as *mut ExtentCallbackData;
                (*data).extents = Box::from(rs_entries);
            }
        }
        Err(e) => {
            panic!("something's wrong! error: {}", e);
        }
    }

    return 0;
}
